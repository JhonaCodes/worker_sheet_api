# Comandos Básicos de Diesel CLI

## Comandos de Instalación
```bash
# Instalar Diesel CLI solo para PostgreSQL
cargo install diesel_cli --no-default-features --features postgres
```

## Comandos Principales

### 1. Setup
```bash
# Inicializa Diesel en tu proyecto
diesel setup
```
**¿Qué hace?**
- Crea la base de datos si no existe
- Crea la carpeta `migrations/`
- Crea el archivo `diesel.toml`
- Crea la tabla `__diesel_schema_migrations` para tracking

### 2. Migraciones
```bash
# Crear nueva migración
diesel migration generate nombre_migracion
```
**¿Qué hace?**
- Crea una carpeta con timestamp en `migrations/`
- Genera dos archivos:
    - `up.sql`: Para crear/modificar
    - `down.sql`: Para revertir cambios

```bash
# Ejecutar migraciones pendientes
diesel migration run
```
**¿Qué hace?**
- Ejecuta las migraciones que no se han aplicado
- Actualiza la tabla `__diesel_schema_migrations`
- Genera/actualiza `schema.rs`

```bash
# Revertir última migración
diesel migration revert
```
**¿Qué hace?**
- Ejecuta el último archivo `down.sql`
- Elimina el registro de la migración

```bash
# Rehacer última migración
diesel migration redo
```
**¿Qué hace?**
- Revierte la última migración
- La vuelve a aplicar
- Útil para desarrollo y pruebas

### 3. Base de Datos
```bash
# Resetear la base de datos
diesel database reset
```
**¿Qué hace?**
- Elimina la base de datos
- La crea nuevamente
- Ejecuta todas las migraciones

```bash
# Eliminar la base de datos
diesel database drop
```
**¿Qué hace?**
- Elimina la base de datos completamente

### 4. Schema
```bash
# Regenerar schema.rs
diesel print-schema > src/schema.rs
```
**¿Qué hace?**
- Lee la estructura de la base de datos
- Genera el archivo schema.rs actualizado

### 5. Estado de Migraciones
```bash
# Ver estado de migraciones
diesel migration list
```
**¿Qué hace?**
- Muestra todas las migraciones
- Indica cuáles están aplicadas

```bash
# Verificar migraciones
diesel migration verify
```
**¿Qué hace?**
- Comprueba que las migraciones estén sincronizadas
- Útil para detectar problemas

## Uso con Variables de Entorno
```bash
# Usar una base de datos específica
DATABASE_URL=postgres://user:pass@localhost/dbname diesel migration run

# Usar un archivo .env diferente
diesel --env-file=".env.test" migration run
```

## Flujo de Trabajo Típico

1. **Inicio del Proyecto**
```bash
diesel setup
```

2. **Nueva Característica/Cambio**
```bash
diesel migration generate create_users
# Editar up.sql y down.sql
diesel migration run
```

3. **Corregir Migración**
```bash
diesel migration redo
```

4. **Actualizar Schema**
```bash
diesel print-schema > src/schema.rs
```

5. **Revertir Cambios (si es necesario)**
```bash
diesel migration revert
```

## Notas Importantes
- `diesel setup` solo se ejecuta una vez al inicio del proyecto
- `migration run` se usa frecuentemente durante el desarrollo
- `migration redo` es útil mientras desarrollas una migración
- `database reset` es útil para limpiar datos en desarrollo
- `print-schema` se usa cuando cambias la estructura de la base de datos

Recomendaciones:
- Siempre tener respaldo de la base de datos
- Probar migraciones en desarrollo antes de producción
- Mantener actualizados both `up.sql` y `down.sql`