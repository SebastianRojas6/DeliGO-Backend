<h1 align="center" style="font-weight: bold;">DeliGO 🍽️</h1>

<p align="center">
 <a href="#technologies">Technologies</a> • 
 <a href="#started">Getting Started</a> • 
 <a href="#routes">API Endpoints</a> •
 <a href="#colab">Collaborators</a> •
 <a href="#contribute">Contribute</a>
</p>

<p align="center">
    <b>Plataforma de pedidos en línea para restaurantes con reparto en tiempo real.</b><br>
    <b>Los usuarios pueden pedir comida, hacer seguimiento del estado y ubicación, y calificar repartidores.</b>
</p>

---

<h2 id="technologies">💻 Technologies</h2>

- Actix Web
- SeaORM
- PostgreSQL (Supabase)
- MongoDB
- Google Maps API

---

<h2 id="started">🚀 Getting started</h2>

Aquí te mostramos cómo correr el proyecto en local.

<h3>Prerequisites</h3>

- [Rust](https://www.rust-lang.org/tools/install)
- [Docker](https://www.docker.com/)
- [Git](https://git-scm.com/)
- [Supabase CLI](https://supabase.com/docs/guides/cli)

<h3>Cloning</h3>

Clona el repositorio de DeliGO en tu máquina local:

```bash
git clone https://github.com/SebastianRojas6/DeliGO-Backend.git
cd deligo
```

<h3>Config .env variables</h3>
Usa el archivo .env.example como referencia para crear tu archivo de configuración .env con tus credenciales:

DATABASE_URL=postgres://usuario:contraseña@localhost:5432/deligo
MONGO_URL=mongodb://localhost:27017
SUPABASE_URL=https://<your-supabase-id>.supabase.co
SUPABASE_KEY=your-anon-key
MAPS_API_KEY=your-google-maps-api-key


<h3>Starting</h3>
Cómo iniciar el proyecto localmente:

```bash
# Ejecutar backend directamente con cargo
cargo run

# O usar Docker (si tienes configurado docker-compose.yml)
docker-compose up --build
```
