# Auditoría y curación de Arsenalero MCP

**Documento auditado:** Diseño conceptual curado v1.1 aportado por el usuario  
**Resultado:** Aprobable con correcciones estructurales  
**Versión resultante:** SDD v1.3  
**Marco:** *AI Engineering* de Chip Huyen + Constitución de Código Agéntico v1.0

---

## 1. Veredicto

El informe v1.1 mejora de forma importante el borrador inicial. En particular, corrige cuatro defectos reales:

1. desacopla el conjunto REQUIRED de las etapas que el agente decide reportar;
2. introduce un techo de confianza por recurso;
3. distingue la entrega del recurso de la evidencia de uso;
4. maneja cambios del recurso durante una sesión.

Sin embargo, no debía pasar directamente a implementación. Contenía contradicciones que podían producir una ilusión de seguridad:

- declaraba cinco tools y añadía una sexta;
- usaba `verified_completion` sin disponer de un verificador confiable;
- convertía toda clasificación DERIVED en obligación REQUIRED;
- prometía renderizado automático al final de la sesión sin un mecanismo del host que lo garantizara;
- aceptaba artefactos como evidencia sin definir si el servidor podía leerlos o validarlos.

La versión v1.3 conserva lo útil y elimina esas afirmaciones excesivas.

---

## 2. Principios de AI Engineering aplicados

### 2.1 Evaluar el sistema, no solo un componente

Arsenalero no debe evaluarse únicamente por si sus tools responden correctamente. La unidad real es:

```text
skill + instrucciones + agente + MCP + recursos + resultado
```

Por eso la evaluación se divide en:

- inventario;
- clasificación;
- selección y entrega;
- receipts y estado;
- reconciliación;
- comportamiento del agente;
- resultado de la tarea;
- costo y latencia añadidos.

### 2.2 Empezar simple y añadir complejidad solo ante evidencia

Se mantienen fuera del MVP:

- LLM interno;
- embeddings;
- RAG;
- hooks;
- base de datos;
- ejecución de scripts;
- validadores arbitrarios;
- UI propia;
- selección de skills.

Esta exclusión no es conservadurismo. Es control de variables. Antes de añadir enforcement o validación semántica hay que demostrar que inventariar, entregar y reconciliar reduce omisiones.

### 2.3 El contexto es una variable de calidad y costo

Cargar todos los resources siempre resolvería el problema de omisión, pero destruiría el propósito de progressive disclosure.

Arsenalero debe medir:

- bytes entregados;
- número de resources emitidos;
- recursos innecesarios;
- tool calls añadidas;
- latencia;
- éxito final de la tarea.

La meta no es maximizar lecturas, sino mejorar la precisión contextual.

### 2.4 Los agentes fallan al planificar y al usar tools

Un agente puede:

- no llamar al MCP;
- declarar una etapa incorrecta;
- pedir el resource equivocado;
- omitir una atestación;
- atribuir evidencia irrelevante;
- leer directamente el archivo y eludir el circuito.

La SDD no oculta estos límites. La reconciliación informa omisiones, pero no afirma enforcement.

### 2.5 Las tools deben ser pocas, claras y diferenciadas

Cinco tools son suficientes. `arsenal_case_summary` no se incorpora.

`arsenal_reconcile` es idempotente y puede utilizarse tanto para un resumen parcial como para el recuento final. Añadir otra tool con información superpuesta aumenta ambigüedad y carga de selección.

### 2.6 Los graders deben corresponder a lo que realmente miden

- Checks de rutas, hashes y receipts: grader determinista.
- Clasificación REQUIRED: grader contra dataset humano etiquetado.
- Calidad del resultado de la tarea: grader funcional y, cuando no sea automatizable, revisión humana calibrada.
- Comprensión del resource: no evaluable directamente.

---

## 3. Cambios aceptados sin modificación sustancial

| Elemento del informe v1.1 | Decisión |
|---|---|
| Un MCP global reutilizable | Aceptado |
| Skill ya activada como precondición | Aceptado |
| Sin `harness.toml` obligatorio | Aceptado |
| Inventario con rutas y SHA-256 | Aceptado |
| Conjunto REQUIRED fijado en `arsenal_init` | Aceptado con nueva regla de obligación |
| `arsenal_stage` no modifica REQUIRED | Aceptado |
| Batch en issue y attest | Aceptado con límites estrictos |
| Receipt stale antes del attest | Aceptado |
| Cambio posterior al attest como evento separado | Aceptado con estado `NeedsReview` |
| Journal JSONL | Aceptado |
| Sin DB, hooks, LLM interno o ejecución arbitraria | Aceptado |
| Trust ceiling explícito | Aceptado |

---

## 4. Cambios modificados

### 4.1 Clasificación y obligación quedan separadas

El informe mezclaba dos preguntas:

1. ¿Cómo se descubrió el significado del resource?
2. ¿Es obligatorio utilizarlo?

En v1.3 son campos independientes:

```text
classification_source:
  DECLARED | DERIVED | UNRESOLVED

obligation:
  REQUIRED | RECOMMENDED | OPTIONAL | UNKNOWN
```

Un resource puede ser `DERIVED + RECOMMENDED`.

Solo forma parte de REQUIRED cuando:

- metadata explícita declara `requirement: required`; o
- la oración o ítem adyacente contiene una marca normativa inequívoca, como `must`, `required`, `before completing`, `debe`, `obligatorio` o `antes de finalizar`.

La coincidencia con una etapa y un verbo de propósito no basta para convertirlo en obligatorio.

**Razón:** usar inferencia estructural como obligación generaría falsos incompletos y degradaría la confianza del sistema.

### 4.2 Dos métricas, pero sin llamar verificado a lo no verificado

La salida final contiene:

```text
protocol_completion
evidence_coverage
verification
```

`protocol_completion` mide REQUIRED, ISSUED y ATTESTED.

`evidence_coverage` mide cuántos recursos que esperaban evidencia tienen una referencia de evidencia registrada.

`verification` en v1 siempre informa:

```json
{
  "status": "not_supported_in_v1",
  "verified_resources": 0
}
```

No existe `verified_completion` numérico hasta integrar un validador externo confiable mediante una SDD separada.

**Razón:** una ruta como `verification-report.json` es una referencia, no una prueba de que el archivo exista, corresponda al receipt o haya pasado una validación.

### 4.3 Modificación posterior a la atestación

No invalida retrospectivamente que el agente haya recibido una versión concreta, pero impide cerrar como `Complete`.

Resultado:

```text
RESOURCE_MODIFIED_POST_ATTESTATION
→ ReconciliationStatus::NeedsReview
```

Así se conserva la evidencia histórica sin ocultar que la fuente de verdad cambió.

### 4.4 Batch con presupuesto de contexto

`arsenal_issue` permite como máximo:

- 4 resources por llamada;
- 256 KiB agregados por llamada;
- 2 MiB agregados por caso.

Cada resource conserva receipt propio.

`arsenal_attest` acepta hasta 16 atestaciones por llamada.

### 4.5 Reporte visible, no auto-renderizado

El MCP devuelve:

- `structuredContent`;
- una representación textual breve.

La skill debe instruir al agente para presentar el recuento al usuario. El MCP por sí solo no puede garantizar que el host muestre un panel final automático.

Una UI automática queda diferida.

---

## 5. Cambios rechazados

### 5.1 Sexta tool `arsenal_case_summary`

Rechazada en v1.

Motivos:

- contradice la promesa de cinco tools;
- solapa `arsenal_reconcile`;
- aumenta la carga de selección;
- no añade una capacidad distinta.

### 5.2 REQUIRED igual a DECLARED o DERIVED

Rechazado.

`DECLARED` describe el origen de la clasificación, no necesariamente obligación. `DERIVED` tampoco puede escalar por sí solo a requisito.

### 5.3 Validación externa implícita

Rechazada.

El MVP no lee artefactos del workspace, no ejecuta comandos y no integra un validador externo. Por tanto no puede declarar `validated`.

### 5.4 Inferencia por “equivalentes razonables”

Rechazada por no ser reproducible.

La versión v1.3 incluye tablas cerradas de aliases bilingües. Cualquier término fuera de ellas produce `UNRESOLVED` o `RECOMMENDED`, no una interpretación creativa.

### 5.5 Detección libre de cualquier nombre de archivo

Rechazada por exceso de falsos positivos.

V1 descubre:

- enlaces Markdown relativos;
- rutas colocadas entre backticks;
- rutas con prefijo `resources/` o `references/`.

Una mención libre como “revisa architecture.md” sin sintaxis de ruta se registra como candidato no resuelto, pero no entra automáticamente al inventario REQUIRED.

---

## 6. Auditoría de seguridad

### Aciertos

- `stdio`;
- sin red;
- sin ejecución;
- skill read-only;
- journal fuera de la skill;
- canonicalización de rutas;
- receipts ligados a hash y caso.

### Controles añadidos

1. `task_summary` es informativo y no participa en la clasificación.
2. El digest de `SKILL.md` se vuelve parte del caso.
3. Si cambia `SKILL.md` después de `init`, el caso queda `Invalidated`.
4. Symlinks que escapan de la raíz se rechazan.
5. El servidor requiere raíces permitidas explícitas.
6. El contenido de resources se trata como datos no confiables y nunca se ejecuta.
7. Los errores de dominio se devuelven como tool execution errors accionables.

---

## 7. Evaluación curada

### 7.1 Regression evals

Deben acercarse a 100 %:

- path traversal;
- symlink escape;
- broken links;
- digest drift;
- receipt cross-case;
- state transitions;
- journal ordering;
- reconciliation exacta.

### 7.2 Capability evals

Buscan descubrir el límite del sistema:

- estilos desconocidos de `SKILL.md`;
- encabezados no canónicos;
- referencias ambiguas;
- recursos con propósitos múltiples;
- mezcla de español e inglés;
- documentos demasiado extensos.

No se espera 100 %. Los errores deben alimentar mejoras de metadata o parsing, no un LLM interno.

### 7.3 Evaluación end-to-end en tres brazos

```text
A. Skill original
B. Skill con instrucciones explícitas, sin MCP
C. Skill con instrucciones + Arsenalero MCP
```

Esto separa el valor del MCP del valor de simplemente mejorar el texto de la skill.

### 7.4 Métricas primarias

- task success;
- required-resource issuance recall;
- required-resource attestation recall;
- false-complete rate;
- required-set precision y recall contra etiquetas humanas.

### 7.5 Métricas de eficiencia

- tool calls añadidas;
- bytes de resources entregados;
- unnecessary-resource issuance rate;
- latencia p50/p95 por tool;
- tiempo total hasta reconciliación.

### 7.6 Repetición

Los casos end-to-end se ejecutan al menos tres veces.

Se reportan:

- `pass@3`: posibilidad de éxito en tres intentos;
- `pass^3`: probabilidad de que los tres intentos resulten correctos.

Para confiabilidad operacional, `pass^3` es la métrica más exigente.

---

## 8. Go / no-go

### GO para implementación del MVP cuando

- SDD v1.3 aprobada;
- plan v1.3 aprobado;
- Context7 disponible en el entorno Codex;
- tool schemas confirmados contra SDK vigente;
- dataset inicial de fixtures etiquetado;
- no se añaden tools, hooks o validadores fuera del plan.

### NO-GO cuando

- se pretende llamar `verified` a una atestación;
- se usa un LLM para decidir REQUIRED;
- el servidor obtiene permisos de ejecución;
- el plugin intenta auto-activarse o seleccionar skills;
- el diff incorpora UI o enforcement antes de demostrar mejora end-to-end;
- la implementación depende de APIs no verificadas con Context7 y compilación.

---

## 9. Conclusión

El informe v1.1 aportó una mejora conceptual real, especialmente el conjunto REQUIRED independiente de `arsenal_stage` y el techo de confianza.

La curación v1.3 elimina cuatro formas de falsa certeza:

- obligación inferida sin norma explícita;
- verificación sin verificador;
- UI automática sin soporte del host;
- sexta tool disfrazada como auxiliar.

El resultado conserva la analogía del arsenalero:

> inventario inicial, entrega trazable y recuento final; nunca diagnóstico, decisión quirúrgica ni certificación de comprensión.
---

## 10. Segunda revisión: hallazgos incorporados en v1.3

La revisión posterior identificó correctamente que el modelo temporal de evidencia seguía mal nombrado y que el ejemplo aritmético de reconciliación no cerraba.

### 10.1 Obligación

El hallazgo original sobre REQUIRED era correcto para v1.1, pero el remedio de derivar obligación desde el tipo A/B/C fue rechazado.

Razón:

```text
tipo del resource ≠ obligatoriedad
tipo de evidencia ≠ obligatoriedad
```

V1.3 conserva dimensiones separadas:

- `classification_source`;
- `obligation`;
- `resource_kind`;
- `evidence_contract`.

Solo una declaración normativa explícita puede producir REQUIRED.

### 10.2 Evidencia temporal

Se elimina `trust_ceiling` de `arsenal_issue`.

El issue devuelve:

```text
evidence_contract.minimum
evidence_contract.supported_levels
```

La atestación calcula:

```text
attained_evidence_level
```

Esto distingue capacidad teórica de evidencia observada.

### 10.3 Aritmética

Se elimina `unverifiable_attested`.

Se reemplaza por:

```text
attestation_breakdown.self_report_only
attestation_breakdown.artifact_referenced
attestation_breakdown.externally_verified
```

Con invariantes validadas en runtime.

### 10.4 Sexta tool, determinismo y dry_run

Estos defectos ya estaban corregidos en v1.2:

- exactamente cinco tools;
- `arsenal_reconcile` reemplaza summary;
- vocabularios cerrados;
- `dry_run` eliminado.

### 10.5 Ejemplos abreviados

Los arrays parciales deben declararse explícitamente como abreviados. El contrato real devuelve una entrada por resource.
