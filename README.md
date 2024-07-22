# Reminders worker
Estos cronjobs son genericos y estan asociados a consumir un endpoint del bot asociado a recordatorios y tareas varias

## Configuración

### Requisitos
Este proyecto esta realizado con los workers de cloudflare por lo que necesitaras tener encuenta lo siguiente:

- [wrangler](https://developers.cloudflare.com/workers/wrangler/install-and-update/)
- [worker-build](https://crates.io/crates/worker-build)
    - [wasm-pack](https://rustwasm.github.io/wasm-pack/)

### Variables de Entorno
Basarse en el archivo `.dev.vars.example` el cual tiene las siguientes variables de entorno:

- `BOT_HOST`: Nombre el host del bot (incluir http o htpps).
- `BOT_API_KEY`: API key necesaria para acceder al endpoint del bot.

## Setup
Ejecutar el cronjob en modo desarrollo
```shell
npx wrangler dev --test-scheduled
```

Para probar el cronjob hacer los siguiente:

```shell
curl "http://localhost:8787/__scheduled?cron=0+12+*+*+5"
```

### Cronjobs disponibles

| Cronjob | Time | Descripcion |
| ---- | ---- | --- |
| english_day | 0 10 * * 5 | Activa el endpoint para publicar el aviso |
| good_morning | 0 13 * * * | Activa el endpoint para publicar el saludo |
| good_night | 0 3 * * * | Activa el endpoint para publicar el saludo |
| wallet_refill | 0 17 * * 6 | Activa el endpoint para hacer el refill de la wallet |

> Nota: el puerto del worker puede ser diferente
> Los tiempos de los cronjobs estan en UTC
