# Context7 Evidence Protocol

**Aplicación:** implementación de Arsenalero MCP v1.3  
**Naturaleza:** gate previo a código dependiente de librerías  
**Runtime:** no aplica

---

## 1. Propósito

Evitar:

- APIs inventadas;
- ejemplos obsoletos;
- versiones incompatibles;
- uso de features no disponibles;
- dependencia de memoria del agente.

Context7 aporta documentación vigente y específica por versión. No reemplaza:

- la especificación MCP;
- el compilador;
- tests;
- `Cargo.lock`;
- revisión humana.

---

## 2. Regla obligatoria

No se escribe código que use una API externa hasta completar:

```text
resolve
→ query
→ record
→ test
→ implement
→ compile
```

---

## 3. Ledger

Archivo:

```text
docs/evidence/context7-ledger.md
```

Plantilla:

```markdown
## YYYY-MM-DD — <library>

- Requested package:
- Resolved Context7 library ID:
- Requested version:
- Query:
- Contract learned:
- Chosen API:
- Rejected alternatives:
- Security implications:
- Files affected:
- Verification command:
- Result:
```

---

## 4. Consultas iniciales obligatorias

### 4.1 Rust MCP SDK

Resolver la librería oficial del SDK Rust de MCP.

Consulta:

```text
For the selected stable rmcp version, show how to build a tools-only
MCP server over stdio with five static tools, typed parameters,
generated inputSchema and outputSchema, structuredContent plus
TextContent compatibility, and tool execution errors with isError=true.
Do not include resources, prompts, sampling, roots, HTTP transport,
or dynamic tool registration.
```

Verificar:

- versión estable;
- feature flags;
- macros;
- `stdio`;
- lifecycle;
- output schemas;
- error mapping;
- concurrency model.

### 4.2 Tokio

Consulta:

```text
For the pinned Tokio version, show the minimal async main and graceful
stdio server shutdown pattern required by rmcp. Include cancellation and
process termination behavior. Exclude network runtimes.
```

### 4.3 Serde

Consulta:

```text
For the pinned Serde version, show strict JSON deserialization for tagged
enums, denial of unknown fields where appropriate, and serialization of
stable reason-code payloads.
```

### 4.4 Schemars

Consulta:

```text
For the pinned Schemars version used by rmcp, show generation of JSON
Schema 2020-12 for nested input and output structs, enums, arrays with
maxItems, and field descriptions.
```

### 4.5 Markdown parser

Primero comparar candidatos con documentación vigente.

Consulta:

```text
For the selected Rust Markdown parser, show how to stream events for
headings, list items, inline code, and relative links while preserving
source ranges. The parser must not execute HTML, scripts, links, or
embedded code.
```

Criterio:

- bajo número de dependencias;
- source offsets;
- CommonMark suficiente;
- no rendering requerido.

### 4.6 SHA-256

Consulta:

```text
For the pinned sha2 version, show streaming SHA-256 over a file without
loading the complete file into memory and stable lowercase hex encoding.
```

### 4.7 UUIDv7

Consulta:

```text
For the pinned uuid version, show UUIDv7 generation, serde support, and
thread-safe use. Confirm feature flags.
```

### 4.8 Directorio de datos

Consulta:

```text
For the selected cross-platform directory crate, show how to locate a
per-user application data directory on macOS, Linux, and Windows without
writing inside the project or skill.
```

### 4.9 Property testing

Consulta:

```text
For the pinned proptest version, show filesystem-independent property
tests for normalized relative paths, legal state transitions, and
receipt-case isolation.
```

### 4.10 Dependency policy

Consulta:

```text
For cargo-deny, show current configuration for advisories, bans,
licenses, and sources for a small Rust workspace.
```

---

## 5. Stop conditions

La implementación se detiene cuando:

- Context7 no puede resolver la librería oficial;
- documentación y crate seleccionado discrepan;
- la API requerida solo existe en una rama inestable;
- la dependencia exige red o ejecución no prevista;
- la versión no genera schemas compatibles;
- el ejemplo no compila tras fijar versiones;
- se requiere ampliar el alcance de la SDD.

---

## 6. Evidencia mínima por dependencia

Cada dependencia debe dejar:

1. entrada en ledger;
2. versión fijada;
3. test que cubra el contrato utilizado;
4. comando de compilación;
5. resultado;
6. nota de seguridad cuando toca filesystem, procesos o serialización.

---

## 7. Limitación de esta documentación

En la sesión donde se redactó esta SDD no estuvo disponible una tool Context7 invocable. Por eso no se fijan versiones ni firmas definitivas de librerías desde memoria.

La implementación debe ejecutar estas consultas dentro de Codex con Context7 antes de escribir el código dependiente.
