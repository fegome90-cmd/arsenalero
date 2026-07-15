# Informe de diseño aportado por el usuario

## Arsenalero MCP — Diseño conceptual curado

**v1.1 – refinamientos sobre el análisis crítico**

---

## 1. Definición

Arsenalero es un servidor MCP local que administra el ciclo de vida de los recursos de una skill ya activada.
Su responsabilidad empieza después de que el agente entra en una skill y termina cuando se reconcilian los recursos utilizados.
No selecciona skills, no activa workflows, no ejecuta el trabajo principal y no gobierna al agente.

Función: Inspeccionar la skill activa, identificar su procedimiento y recursos, entregar los recursos correspondientes a cada etapa, registrar evidencia de utilización y producir un recuento final.

La analogía quirúrgica se mantiene:

| Elemento | Rol |
|---|---|
| Usuario / sistema de routing | Decide si habrá cirugía |
| Skill | Define la intervención |
| Agente | Cirujano |
| Arsenalero MCP | Arsenalero |
| Resources | Instrumental e insumos |
| Resultado final | Resultado quirúrgico |
| Reconciliación | Recuento final |

---

## 2. Decisión arquitectónica principal

Un único MCP Arsenalero global, instalado mediante plugin de Codex.
Las skills no contienen: binarios, scripts del harness, servidores MCP propios, manifiestos obligatorios separados ni estado de ejecución.

Estructura de skill:

```text
skill/
├── SKILL.md
└── resources/
    ├── architecture.md
    ├── validation.md
    ├── error-handling.md
    └── examples.md
```

El MCP expone herramientas genéricas capaces de operar con cualquier skill compatible. El plugin distribuye el servidor y lo reutiliza para todas las skills.

---

## 3. Límite de responsabilidad

### Sí hace

1. Recibir ubicación de skill activa.
2. Leer SKILL.md.
3. Localizar referencias a archivos.
4. Inspeccionar cada recurso (ruta, hash, tipo).
5. Construir inventario con clasificación determinista.
6. Relacionar recursos con etapas del procedimiento (según clasificación).
7. Entregar contenido solicitado.
8. Emitir receipts verificables con techo de confianza explícito.
9. Registrar evidencia asociada al uso (atribución).
10. Revisar el inventario al finalizar y generar reporte de dos métricas.
11. Informar omisiones, recursos sin evidencia y cambios posteriores al issue.

### No hace

- Decidir qué skill activar ni si la tarea debe ejecutarse.
- Reemplazar instrucciones de la skill ni redactar/modificar el resultado principal.
- Evaluar calidad general del trabajo.
- Ejecutar comandos declarados dentro de un resource.
- Controlar todas las herramientas del agente.
- Impedir que el agente lea directamente un archivo.
- No afirma que el modelo comprendió un documento ni prueba obediencia absoluta.

---

## 4. Contrato mínimo con la skill

El único requisito obligatorio en SKILL.md es:

```markdown
## Resource handling
After activating this skill:
1. Call `arsenal_init` with this skill directory.
2. Inform Arsenalero when entering each workflow stage (`arsenal_stage`).
3. Obtain referenced resources through `arsenal_issue`.
4. Register their application with `arsenal_attest`.
5. Call `arsenal_reconcile` before completing the workflow.
```

No se requiere harness.toml.

---

## 5. Cómo funciona arsenal_init

Entrada:

```json
{
  "skill_root": "/absolute/path/to/skill",
  "task_summary": "Implement the requested feature"
}
```

### 5.1 Identificación de la skill

Extrae de SKILL.md: frontmatter, nombre, descripción, encabezados, secuencia de pasos, checklists, enlaces Markdown, menciones de archivos en texto plano y frases próximas.

### 5.2 Identificación de los recursos

Para cada archivo obtiene ruta canónica, SHA-256, tamaño, tipo MIME inferido, título, encabezados, primer párrafo, sección, instrucción adyacente, etapa probable, función probable y certeza.

### 5.3 Clasificación determinista

- DECLARED: metadata explícita con purpose y opcionalmente stages.
- DERIVED: heading de etapa conocido y verbo de propósito reconocido.
- UNRESOLVED: cualquier otro caso.

### 5.4 REQUIRED

Todos los resources DECLARED o DERIVED asociados a al menos una etapa.

---

## 6. Metadata opcional

```yaml
---
arsenal:
  id: rollback-procedure
  purpose: Restore the repository after a failed migration.
  stages:
    - implementation
    - recovery
  evidence: acknowledgement
---
```

Jerarquía: metadata → instrucción adyacente → estructura → nombre.

---

## 7. Tools

### 7.1 arsenal_init

Incluye `dry_run`, required IDs, unresolved y orphan files.

### 7.2 arsenal_stage

Sugiere required, recommended y unresolved relevantes.

### 7.3 arsenal_issue

Acepta batch y devuelve receipt, digest, purpose, content, expected evidence y trust ceiling.

### 7.4 arsenal_attest

Acepta batch. Rechaza stale pre-attest y avisa cambio post-attest.

### 7.5 arsenal_reconcile

Entrega protocol completion, verified completion, unverifiable attested, missing, required never issued, stale receipts, post-attestation modifications, unresolved, trust y disclaimer.

---

## 8. Resources MCP frente a tools

Arsenalero usa principalmente tools. Recursos vía URI son opcionales.

---

## 9. Evidencia

- Tipo A: self-report.
- Tipo B: self-report o artifact-backed.
- Tipo C: artifact-backed y validated.

Estados:

```text
DISCOVERED → CLASSIFIED → REQUIRED → ISSUED → ATTESTED → EVIDENCED → VALIDATED → RECONCILED
```

---

## 10. Garantías

Trazabilidad y reconciliación, no cognición u obediencia absoluta.

---

## 11. Estado

Journal JSONL y tool auxiliar `arsenal_case_summary`.

---

## 12. Seguridad

Restricciones previas más detección de cambios.

---

## 13. MVP

Cinco tools más `case_summary`, clasificación determinista, REQUIRED, receipts, attestations, dos métricas, journal y consumidor explícito.

---

## 14. Reason codes

Incluye `RECEIPT_STALE` y `RESOURCE_MODIFIED_POST_ATTESTATION`.

---

## 15. Evals

Incluye casos límite de clasificación determinista.

---

## 16. Aceptación

Incluye distinción de protocol completion y verified completion.

---

## 17. Decisión final

Desacoplamiento de etapas, dos métricas, techo epistémico, clasificación determinista, drift y consumidor del reporte.

La UI de Codex renderiza el reporte de reconciliación al final. Futuro CI falla solo por Tipo C.

---

## Resumen de cambios del informe

- REQUIRED calculado en init.
- Clasificación determinista.
- Dos métricas y trust ceiling.
- Stale pre-attest y warning post-attest.
- Nuevo reason code.
- Consumidor del reporte.
- Batches.
- `stage` no define REQUIRED.
