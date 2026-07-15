# Revisión crítica incorporada en Arsenalero MCP v1.3

## Estado de los hallazgos

| Hallazgo | Estado |
|---|---|
| REQUIRED confundía clasificación y obligatoriedad | Correcto para v1.1; ya corregido en v1.2 y reforzado en v1.3 |
| `recommended_now` sin fuente | Corregido mediante obligación ortogonal |
| Sexta tool `arsenal_case_summary` | Ya corregido: exactamente cinco tools |
| “Equivalentes razonables” y `etc.` | Ya corregido con vocabularios cerrados |
| `trust_ceiling` antes de attest | Correcto; corregido en v1.3 |
| Ejemplo aritmético inconsistente | Correcto; corregido en v1.3 con invariantes runtime |
| `dry_run` sin semántica | Ya eliminado |
| Naming de evidencia inconsistente | Corregido como `evidence_contract` y `attained_evidence_level` |
| Array parcial no marcado | Corregido con nota explícita |

## Decisión no aceptada literalmente

No se adopta esta regla:

```text
Tipo C → required por defecto
Tipo B → required por defecto
Tipo A → recommended por defecto
```

Motivo: el tipo de resource y el tipo de evidencia siguen siendo propiedades distintas de la obligatoriedad.

Ejemplos:

- un schema puede ser un ejemplo opcional;
- una checklist puede ser recomendada;
- una advertencia consultiva de seguridad puede ser obligatoria.

Regla final:

```text
REQUIRED
= metadata requirement: required
  OR marcador normativo cerrado en contexto
```

Todo lo demás queda `RECOMMENDED`, `OPTIONAL` o `UNKNOWN`.
