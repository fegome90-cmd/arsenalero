# Arsenalero MCP — Software Design Document

**Versión:** 1.3  
**Estado:** Proposed for human review  
**Fecha:** 2026-07-15  
**Riesgo:** Medio  
**Host inicial:** Codex mediante plugin  
**Servidor:** Rust, MCP local por `stdio`  
**Constitución:** Constitución de Código Agéntico v1.0

---

## 1. Definición

Arsenalero es un servidor MCP local que administra el ciclo de vida de los resources de una skill ya activada.

Su responsabilidad comienza cuando la skill activa llama `arsenal_init` y termina cuando `arsenal_reconcile` produce el recuento del caso.

Arsenalero no:

- selecciona skills;
- activa workflows;
- ejecuta el trabajo principal;
- controla al agente;
- intercepta otras herramientas;
- afirma que el modelo comprendió un resource.

Su función es:

```text
inspeccionar
→ inventariar
→ clasificar
→ entregar
→ atribuir evidencia
→ reconciliar
```

---

## 2. Analogía operativa

| Elemento | Rol |
|---|---|
| Usuario o sistema de routing | Decide si habrá cirugía |
| Skill | Define la intervención |
| Agente | Cirujano |
| Arsenalero MCP | Arsenalero |
| Resources | Instrumental e insumos |
| Resultado principal | Resultado quirúrgico |
| Reconciliación | Recuento final |

El arsenalero no decide operar ni evalúa el resultado clínico. Se responsabiliza del instrumental bajo su custodia.

---

## 3. Problema

El patrón de skills utiliza un `SKILL.md` central breve que referencia múltiples resources. Este patrón reduce contexto inicial, pero no garantiza que:

- todas las referencias sean válidas;
- el resource apropiado sea entregado;
- exista traza de la entrega;
- el resource siga siendo el mismo al cerrar;
- el agente declare cómo lo utilizó.

Un linter resuelve integridad estática. Arsenalero añade trazabilidad de runtime.

---

## 4. Objetivos

### G1. Un servidor global

Una instalación debe operar con múltiples skills compatibles.

### G2. No asumir responsabilidad de activación

La skill activa debe solicitar Arsenalero explícitamente.

### G3. Inventario determinista

Rutas, hashes, clasificación, obligaciones y receipts deben ser reproducibles.

### G4. Migración mínima

No se requiere `harness.toml`. La metadata es opcional.

### G5. Evidencia honesta

El sistema distingue:

- entrega;
- atestación;
- referencia de evidencia;
- verificación externa.

### G6. Contexto eficiente

No se cargan todos los resources indiscriminadamente. Se miden costo y selección.

### G7. Seguridad local

Sin red, shell, ejecución de scripts ni escrituras dentro de la skill.

### G8. Núcleo neutral

La lógica de dominio no depende de Codex ni del transporte MCP.

---

## 5. Fuera de alcance

El MVP no incluye:

- routing o activación de skills;
- LLM interno;
- embeddings o RAG;
- AST, LSP o graph;
- hooks;
- tool masking;
- ejecución de validadores;
- lectura de artefactos del workspace;
- base de datos;
- listener de red;
- UI propia;
- CI gate;
- sincronización remota;
- tools dinámicas;
- prueba de comprensión u obediencia.

---

## 6. Decisión arquitectónica

Se implementa un MCP global instalado mediante plugin.

```text
Codex
  |
  | MCP stdio
  v
arsenalero-mcp
  |
  v
arsenalero-core
  |
  +-- skill root: read-only
  +-- plugin data: append-only journal
```

### 6.1 Componentes

#### `arsenalero-core`

Responsable de:

- contratos de dominio;
- path policy;
- scanner;
- clasificación;
- casos;
- receipts;
- journal;
- reconciliación.

No conoce MCP.

#### `arsenalero-mcp`

Responsable de:

- lifecycle MCP;
- cinco tools;
- JSON Schema;
- structured results;
- mapping de errores;
- transporte `stdio`.

#### Plugin Codex

Responsable de:

- distribuir/configurar el servidor;
- hacerlo disponible en sesiones nuevas;
- mantener aprobación de tools en modo `prompt`.

---

## 7. Contrato mínimo con la skill

El `SKILL.md` debe incluir:

```markdown
## Resource handling

After activating this skill:

1. Call `arsenal_init` with this skill directory.
2. Call `arsenal_stage` when entering a workflow stage.
3. Obtain referenced resources through `arsenal_issue`.
4. Register their application with `arsenal_attest`.
5. Call `arsenal_reconcile` before completing the workflow.
6. Surface the reconciliation summary to the user.
```

Esta instrucción no fuerza la activación de la skill. Solo disciplina el manejo de resources una vez activa.

---

## 8. Fuente de verdad

Jerarquía:

1. Constitución de Código Agéntico v1.0.
2. Esta SDD.
3. Especificación MCP.
4. `SKILL.md`.
5. Metadata `arsenal` explícita.
6. Clasificación determinista derivada.
7. Journal de caso como evidencia de ejecución.

El journal no gobierna la skill y no puede modificar su política.

---

## 9. `arsenal_init`

### 9.1 Entrada

```json
{
  "skill_root": "/absolute/path/to/skill",
  "task_summary": "Implement the requested feature",
  "operation": "implementation"
}
```

`task_summary` y `operation` se registran, pero no se usan para decidir REQUIRED.

### 9.2 Inspección de la skill

Extrae:

- frontmatter de `SKILL.md`;
- nombre y descripción;
- headings;
- listas y checklists;
- enlaces Markdown relativos;
- rutas colocadas entre backticks;
- contexto adyacente de cada referencia.

### 9.3 Descubrimiento permitido

V1 descubre como references:

1. `[texto](resources/foo.md)`;
2. `` `resources/foo.md` ``;
3. `` `references/foo.md` ``.

Una mención libre como `foo.md` sin directorio se registra como candidate warning, no como resource confirmado.

### 9.4 Archivos admitidos

- `.md`
- `.txt`
- `.json`
- `.yaml`
- `.yml`
- `.toml`

Límites:

- 128 resources por caso;
- 256 KiB por resource;
- 512 KiB para `SKILL.md`;
- 2 MiB emitidos por caso;
- rutas relativas normalizadas de hasta 256 caracteres.

### 9.5 Controles

- canonicalización;
- raíz allowlisted;
- rechazo de `..`;
- rechazo de symlink que escape;
- archivo regular;
- extensión admitida;
- tamaño admitido;
- SHA-256;
- digest de `SKILL.md`.

---

## 10. Modelo de clasificación

La clasificación tiene dimensiones ortogonales.

### 10.1 Fuente de clasificación

```text
DECLARED
DERIVED
UNRESOLVED
```

#### DECLARED

Existe metadata explícita en el resource o un bloque `arsenal` en `SKILL.md`.

#### DERIVED

Se cumplen:

1. referencia bajo un heading con stage alias reconocido;
2. oración o ítem adyacente con purpose verb reconocido.

#### UNRESOLVED

No cumple las condiciones anteriores o existen señales contradictorias.

### 10.2 Obligación

```text
REQUIRED
RECOMMENDED
OPTIONAL
UNKNOWN
```

#### REQUIRED

Solo cuando:

- metadata declara `requirement: required`; o
- el contexto adyacente contiene un marcador normativo inequívoco.

Marcadores ingleses:

```text
must
required
before completing
before finalizing
do not proceed without
```

Marcadores españoles:

```text
debe
obligatorio
antes de completar
antes de finalizar
no continúe sin
no proceda sin
```

#### OPTIONAL

Metadata o contexto contiene `optional`, `opcional`, `example`, `ejemplo`, `illustrative` o `ilustrativo`.

#### RECOMMENDED

El resource es DECLARED o DERIVED, pero no contiene marcadores REQUIRED u OPTIONAL.

#### UNKNOWN

El resource es UNRESOLVED.

`resource_kind` y `evidence_contract` no determinan obligación. Un procedimiento o contrato verificable puede ser opcional; una referencia consultiva puede ser obligatoria. Solo metadata normativa explícita o lenguaje normativo cerrado puede producir `REQUIRED`.

### 10.3 Stage aliases cerrados

```yaml
exploration:
  - exploration
  - discovery
  - research
  - context
  - exploración
  - descubrimiento
  - investigación
  - contexto

implementation:
  - implementation
  - build
  - coding
  - desarrollo
  - implementación
  - construcción
  - código

testing:
  - testing
  - tests
  - test
  - pruebas
  - prueba

verification:
  - verification
  - validation
  - review
  - verificación
  - validación
  - revisión

recovery:
  - recovery
  - rollback
  - error handling
  - recuperación
  - reversión
  - manejo de errores

deployment:
  - deployment
  - release
  - promotion
  - despliegue
  - liberación
  - promoción
```

No existen “equivalentes razonables” fuera de esta tabla.

### 10.4 Purpose verbs cerrados

```yaml
en:
  - use
  - follow
  - consult
  - check
  - apply
  - validate
  - review
  - run

es:
  - usar
  - utilizar
  - seguir
  - consultar
  - comprobar
  - aplicar
  - validar
  - revisar
  - ejecutar
```

### 10.5 Conjunto REQUIRED

Se fija durante `arsenal_init`.

No depende de llamadas posteriores a `arsenal_stage`.

Cambiar `SKILL.md` invalida el caso porque podría cambiar REQUIRED.

---

## 11. Metadata opcional

Ejemplo:

```yaml
---
arsenal:
  id: rollback-procedure
  purpose: Restore the repository after a failed migration.
  stages:
    - implementation
    - recovery
  requirement: required
  resource_kind: procedure
  evidence_contract:
    minimum: attestation
    supported:
      - attestation
      - artifact_reference
---
```

Campos:

```text
id
purpose
stages
requirement
resource_kind
evidence_contract.minimum
evidence_contract.supported
```

Valores de `resource_kind`:

```text
consultative
procedure
verifiable_contract
```

---

## 12. Tools MCP

El servidor expone exactamente cinco tools.

### 12.1 `arsenal_init`

**Precondición:** skill ya activa.

**Descripción funcional:** abre un caso, valida la raíz, escanea `SKILL.md`, calcula REQUIRED e informa deuda de diseño.

Entrada:

```json
{
  "skill_root": "/path/to/skill",
  "task_summary": "Current task",
  "operation": "verification"
}
```

Salida:

```json
{
  "case_id": "019abc...",
  "skill": {
    "name": "repository-review",
    "digest": "sha256:..."
  },
  "required_resource_ids": [
    "resources::validation"
  ],
  "resources": [],
  "unresolved": [],
  "orphan_files": [],
  "warnings": [],
  "status": "ready"
}
```

### 12.2 `arsenal_stage`

**Función:** devuelve resources relevantes para una etapa declarada.

No altera REQUIRED.

Entrada:

```json
{
  "case_id": "019abc...",
  "stage": "verification",
  "current_intent": "Run final acceptance checks"
}
```

Salida:

```json
{
  "required_now": [],
  "recommended_now": [],
  "already_issued": [],
  "unresolved_relevant": []
}
```

`unresolved_relevant` usa coincidencia léxica exacta de aliases, no similitud semántica.

### 12.3 `arsenal_issue`

**Función:** entrega uno a cuatro resources.

Entrada:

```json
{
  "case_id": "019abc...",
  "resource_ids": [
    "resources::validation",
    "resources::common-failures"
  ]
}
```

Reglas:

- máximo 4 IDs;
- máximo 256 KiB de contenido agregado;
- receipt individual;
- no reissue automático si digest cambió;
- contenido completo, sin chunking en v1.

Salida por resource:

```json
{
  "receipt_id": "019def...",
  "resource_id": "resources::validation",
  "digest": "sha256:...",
  "purpose": "Define final acceptance checks",
  "content": "...",
  "evidence_contract": {
    "minimum": "attestation",
    "supported_levels": [
      "attestation",
      "artifact_reference"
    ]
  }
}
```

`arsenal_issue` describe la **capacidad y expectativa de evidencia** del resource. No informa un nivel alcanzado, porque todavía no existe una atestación.

### 12.4 `arsenal_attest`

**Función:** registra atestaciones para receipts emitidos.

Entrada:

```json
{
  "case_id": "019abc...",
  "attestations": [
    {
      "receipt_id": "019def...",
      "usage": "Used to select final test commands.",
      "evidence": [
        {
          "type": "artifact_reference",
          "reference": "verification-report.json"
        }
      ]
    }
  ]
}
```

Reglas:

- máximo 16;
- `usage` no vacío;
- receipt del mismo caso;
- digest recalculado antes de aceptar;
- cambio pre-attestation produce `RECEIPT_STALE`;
- cambio post-attestation se detecta al reconciliar;
- el servidor calcula `attained_evidence_level`:
  - `attestation` cuando solo existe la declaración de uso;
  - `artifact_reference` cuando se adjunta al menos una referencia de artefacto válida;
- `artifact_reference` demuestra atribución a un artefacto nombrado, no existencia, contenido ni validez del artefacto.

### 12.5 `arsenal_reconcile`

**Función:** resumen idempotente y recuento final.

Puede llamarse varias veces. Reemplaza la necesidad de `arsenal_case_summary`.

Entrada:

```json
{
  "case_id": "019abc..."
}
```

Salida:

```json
{
  "status": "needs_review",
  "protocol_completion": {
    "required": 6,
    "issued": 6,
    "attested": 5,
    "ratio": 0.8333
  },
  "evidence_coverage": {
    "expected_artifact_references": 2,
    "artifact_referenced": 1,
    "ratio": 0.5
  },
  "attestation_breakdown": {
    "self_report_only": 4,
    "artifact_referenced": 1,
    "externally_verified": 0
  },
  "verification": {
    "status": "not_supported_in_v1",
    "verified_resources": 0
  },
  "missing_attestations": [
    "resources::rollback-procedure"
  ],
  "required_but_never_issued": [],
  "stale_receipts": [],
  "resource_modifications_post_attestation": [
    "resources::architecture"
  ],
  "unresolved_resources": [],
  "per_resource_evidence": [
    {
      "resource_id": "resources::validation",
      "attained_evidence_level": "artifact_reference",
      "verification_status": "not_supported_in_v1"
    },
    {
      "resource_id": "resources::architecture",
      "attained_evidence_level": "attestation",
      "verification_status": "not_supported_in_v1"
    }
  ],
  "disclaimer": "Protocol completion records issue and attestation events. Artifact references are agent-supplied attributions and are not externally verified in Arsenalero v1."
}
```

El array `per_resource_evidence` contiene una entrada por cada resource inventariado; el ejemplo muestra solo dos entradas por brevedad.

Invariantes internas obligatorias:

```text
attestation_breakdown.self_report_only
+ attestation_breakdown.artifact_referenced
= protocol_completion.attested

attestation_breakdown.externally_verified
<= attestation_breakdown.artifact_referenced

evidence_coverage.artifact_referenced
= attestation_breakdown.artifact_referenced
```

Una violación de estas invariantes es un error interno del servidor y no un estado de reconciliación válido.
---

## 13. Modelo de evidencia

### Tipo A: consultative

Ejemplos:

- explicación;
- arquitectura;
- glosario;
- ejemplos.

Capacidad en v1:

```text
supported_levels = [attestation]
maximum_attained_evidence_level = attestation
```

### Tipo B: procedure

Ejemplos:

- checklist;
- flujo;
- procedimiento.

Capacidad en v1:

```text
supported_levels = [attestation, artifact_reference]
maximum_attained_evidence_level = artifact_reference
```

Sin lectura ni validación del artefacto.

### Tipo C: verifiable contract

Ejemplos:

- schema;
- invariantes;
- formato de salida;
- comandos obligatorios.

Capacidad en v1:

```text
supported_levels = [attestation, artifact_reference]
maximum_attained_evidence_level = artifact_reference
verification = not_supported_in_v1
```

`externally_verified` se difiere.

---

## 14. Estados

### Caso

```text
Initialized
→ Active
→ Complete | Incomplete | NeedsReview | Invalidated
```

### Resource

```text
Discovered
→ Classified
→ Required | Recommended | Optional | Unknown
→ Issued
→ Attested
→ Reconciled
```

No se incluye `Validated` en v1.

---

## 15. Receipts

```json
{
  "receipt_id": "uuidv7",
  "case_id": "uuidv7",
  "resource_id": "resources::validation",
  "resource_digest": "sha256:...",
  "skill_digest": "sha256:...",
  "issued_at": "RFC3339"
}
```

Invariantes:

- receipt único;
- pertenece a un caso;
- no reutilizable;
- ligado a digest del resource;
- ligado al digest de `SKILL.md`;
- digest verificado al attest y al reconcile.

---

## 16. Drift

### Antes de attest

```text
digest actual != receipt digest
→ RECEIPT_STALE
→ attest rechazado
```

### Después de attest

```text
digest actual != receipt digest
→ RESOURCE_MODIFIED_POST_ATTESTATION
→ status NeedsReview
```

### Cambio de SKILL.md

```text
skill digest actual != case skill digest
→ SKILL_DIGEST_CHANGED
→ status Invalidated
```

---

## 17. Persistencia y observabilidad

Estado en memoria y journal en el directorio de datos del plugin:

```text
PLUGIN_DATA/
└── cases/
    └── <case-id>.jsonl
```

Eventos:

```text
case_initialized
stage_entered
resource_issued
resource_attested
receipt_stale
resource_modified_post_attestation
skill_digest_changed
case_reconciled
```

Cada evento contiene:

```json
{
  "sequence": 7,
  "event_type": "resource_attested",
  "timestamp": "RFC3339",
  "payload": {},
  "previous_digest": "sha256:...",
  "event_digest": "sha256:..."
}
```

La cadena detecta corrupción accidental. No se presenta como firma criptográfica ni garantía contra un atacante con escritura local.

---

## 18. Seguridad

### 18.1 Mínimo privilegio

- roots permitidas explícitas;
- skill read-only;
- plugin data write-only;
- sin red;
- sin shell;
- sin ejecución;
- sin secretos;
- sin ports.

### 18.2 Entradas hostiles

El contenido de un resource es dato no confiable.

Arsenalero:

- no obedece instrucciones internas;
- no ejecuta código;
- no sigue URLs;
- no amplía roots;
- no usa `task_summary` para clasificación.

### 18.3 Límites

- timeout por tool;
- límites de tamaño;
- límite de batch;
- límite de casos concurrentes;
- sanitización de salidas;
- tool errors accionables.

---

## 19. Reason codes

```text
SKILL_ROOT_INVALID
SKILL_ROOT_NOT_ALLOWED
SKILL_MD_MISSING
SKILL_MD_TOO_LARGE
SKILL_DIGEST_CHANGED
RESOURCE_REFERENCE_BROKEN
RESOURCE_PATH_ESCAPE
RESOURCE_SYMLINK_ESCAPE
RESOURCE_TYPE_UNSUPPORTED
RESOURCE_TOO_LARGE
RESOURCE_LIMIT_EXCEEDED
RESOURCE_ID_COLLISION
RESOURCE_UNRESOLVED
RESOURCE_CLASSIFICATION_CONFLICT
STAGE_UNKNOWN
CASE_UNKNOWN
CASE_INVALIDATED
RESOURCE_UNKNOWN
RESOURCE_BATCH_LIMIT
RESOURCE_ALREADY_ISSUED
RECEIPT_UNKNOWN
RECEIPT_CASE_MISMATCH
RECEIPT_STALE
RESOURCE_MODIFIED_POST_ATTESTATION
ATTESTATION_EMPTY
EVIDENCE_REFERENCE_INVALID
RECONCILIATION_COMPLETE
RECONCILIATION_INCOMPLETE
RECONCILIATION_NEEDS_REVIEW
RECONCILIATION_INVALIDATED
RECONCILIATION_INVARIANT_VIOLATION
```

Errores de dominio se devuelven como tool execution errors. Errores de protocolo se reservan para requests MCP malformados o tools desconocidas.

---

## 20. MCP output contract

Cada tool define:

- `inputSchema`;
- `outputSchema`;
- `structuredContent`;
- TextContent con JSON serializado para compatibilidad;
- `isError: true` en errores corregibles de dominio.

Las descripciones deben incluir:

- propósito;
- precondición;
- qué no hace;
- cuándo usarla;
- siguiente paso habitual.

---

## 21. Context7 como gate de desarrollo

Context7 no es dependencia runtime.

Antes de escribir código contra una librería:

1. resolver library ID;
2. consultar documentación específica de la versión;
3. registrar el contrato aprendido;
4. fijar versión;
5. escribir test que exprese el contrato;
6. implementar;
7. compilar y contrastar.

Librerías iniciales:

- official Rust MCP SDK / `rmcp`;
- Tokio;
- Serde;
- Schemars;
- parser Markdown seleccionado;
- SHA-256;
- UUIDv7;
- directorios de datos cross-platform;
- property testing;
- dependency audit.

---

## 22. Evaluación

### 22.1 Por componente

#### Scanner

- reference precision;
- reference recall;
- orphan detection;
- broken-link detection.

#### Clasificador

- classification-source accuracy;
- obligation precision;
- obligation recall;
- unresolved rate.

#### Receipt system

- cross-case rejection;
- stale detection;
- skill drift detection.

#### Reconciliación

- false-complete rate;
- missing-resource accuracy;
- status accuracy.

### 22.2 Tres brazos end-to-end

```text
A. Skill original
B. Skill con bloque Resource handling, sin MCP
C. Skill con bloque + Arsenalero
```

### 22.3 Outcomes

Primarios:

- task success;
- required-resource issuance recall;
- required-resource attestation recall;
- false-complete rate.

Secundarios:

- unnecessary-resource issuance;
- tool calls;
- bytes entregados;
- p50/p95 latency;
- total task time.

### 22.4 Capability y regression

Regression:

- determinista;
- fixtures bloqueados;
- objetivo cercano a 100 %.

Capability:

- skills nuevas o ambiguas;
- mide límites y cobertura;
- no se usa como gate binario.

### 22.5 Repetición

End-to-end:

- al menos 3 trials por caso;
- reportar `pass@3`;
- reportar `pass^3`.

---

## 23. CI y quality gates

```bash
cargo fmt --all --check
cargo clippy --workspace --all-targets --all-features -- -D warnings
cargo test --workspace --all-features
cargo deny check
```

Release:

```bash
cargo install --path crates/arsenalero-mcp --locked
arsenalero-mcp --version
```

El MCP stdio integration test es obligatorio.

---

## 24. Criterios de aceptación

1. Un servidor opera con múltiples fixture skills.
2. Expone exactamente cinco tools.
3. No usa red ni shell.
4. Roots fuera de allowlist fallan cerradas.
5. Traversal y symlink escapes fallan.
6. Descubre referencias soportadas con 100 % recall en regression fixtures.
7. No convierte clasificación DERIVED en REQUIRED sin marcador normativo.
8. Ambigüedad produce UNRESOLVED.
9. Cada issue genera receipt individual.
10. Receipts cross-case se rechazan.
11. Drift pre-attest produce stale.
12. Drift post-attest produce NeedsReview.
13. Cambio de `SKILL.md` invalida el caso.
14. Reconcile distingue protocol completion, evidence coverage, attestation breakdown y verification unsupported.
15. No existe una sexta tool.
16. Journal queda fuera de la skill.
17. Context7 ledger cubre todas las APIs externas.
18. Regression suite pasa.
19. En la evaluación de tres brazos, Arsenalero reduce omisiones respecto de A y aporta beneficio superior a B.
20. La sobrecarga de contexto y latencia queda cuantificada.
21. `arsenal_issue` no declara un nivel de evidencia alcanzado.
22. `arsenal_attest` calcula `attained_evidence_level`.
23. Las invariantes aritméticas de reconciliación se validan en runtime.

---

## 25. Criterios de no avance

No promover a piloto real si:

- required-set precision < 0.95;
- false-complete rate > 0 en regression;
- path escape acceptance > 0;
- receipt cross-case acceptance > 0;
- tool contract tests fallan;
- Context7 ledger está incompleto;
- el brazo C no mejora omisiones respecto del brazo B;
- la sobrecarga no está medida;
- aparece ejecución arbitraria, red, hooks o una sexta tool.

---

## 26. Capacidades diferidas

Requieren SDD y aprobación separadas:

- verificación de artefactos;
- workspace read scope;
- CI gate Tipo C;
- UI automática;
- hooks;
- tool masking;
- signed receipts;
- validadores de skills;
- clasificación semántica;
- remote MCP.

---

## 27. Decisión final

Arsenalero v1.3 es:

```text
observer
+ dispenser
+ reconciler
```

Su garantía máxima:

> Para una skill ya activa, puede demostrar qué resources referenciados existían, cuáles eran explícitamente obligatorios, qué versión exacta se entregó, qué uso declaró el agente y si el recuento cerró sin omisiones o drift.

No prueba cognición, obediencia ni corrección semántica.
