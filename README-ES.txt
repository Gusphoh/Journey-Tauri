CLUB PENGUIN JOURNEY - LANZADOR TAURI

Este proyecto abre https://play.cpjourney.net/ en una ventana de escritorio. El ZIP no incluye un .exe compilado.

COMPILAR LOCALMENTE EN WINDOWS
1. Instala Node.js LTS: https://nodejs.org/en/download
2. Instala Rust: https://rustup.rs/
3. Instala los requisitos de Windows de Tauri: https://v2.tauri.app/start/prerequisites/
4. Extrae el ZIP y abre CMD/PowerShell dentro de la carpeta del proyecto.
5. Ejecuta:
   npm install
   npm run tauri:build
6. El instalador debería aparecer en:
   src-tauri\target\release\bundle\nsis\

GENERARLO CON GITHUB ACTIONS (SIN COMPILAR EN TU PC)
1. Crea un repositorio vacío en GitHub y sube TODO el contenido de esta carpeta, incluida la carpeta oculta .github.
2. Abre la pestaña Actions del repositorio.
3. En Actions, selecciona "Build Club Penguin Journey for Windows" y pulsa Run workflow. También se ejecutará al subir cambios a main/master.
4. Cuando termine, abre la ejecución y descarga el artefacto "Club-Penguin-Journey-Windows-Installer".
5. Dentro estará el instalador .exe.

NOTAS
- Se necesita Internet para jugar.
- La compatibilidad y el rendimiento dependen del sitio y de WebView2 en Windows; no se garantiza que supere a Chrome/Brave en FPS.
- Este lanzador no modifica monedas, objetos ni el funcionamiento interno del juego.

IMPORTANTE: el icono ya está incluido en src-tauri/icons/icon.ico y configurado en tauri.conf.json. No lo muevas ni cambies la ruta.
