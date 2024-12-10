Este enfoque se llama **Inversión de Dependencias** (Dependency Inversion Principle), que es uno de los principios SOLID de diseño de software. En este caso, el patrón en particular es una forma de aplicar **Composición** y **Abstracción** para evitar dependencias directas entre clases (o estructuras) concretas.

### Términos clave que se aplican a este diseño:
1. **Inversión de Dependencias**: En lugar de que un objeto dependa directamente de otro (como un préstamo dentro de una persona), la dependencia se invierte para que se dependa de una **abstracción** (un servicio de préstamos, en este caso). Esto permite flexibilidad y facilita la modificación o extensión del sistema sin cambiar las clases consumidoras.
   
2. **Composición sobre Herencia**: Se prefiere usar la composición para agregar funcionalidad a las clases (o estructuras) en lugar de depender de la herencia. Aquí, la `Person` no hereda de `Loan`, sino que interactúa con el `LoanService` para obtener un préstamo.

3. **Abstracción y desacoplamiento**: Al definir el trait `Loan`, se abstrae el comportamiento de un préstamo, lo que permite tener múltiples implementaciones sin que `Person` dependa de las clases concretas. Este desacoplamiento facilita la modificación y escalabilidad del sistema.

4. **Patrón de Diseño**: Es una forma de crear un sistema en el que las dependencias se inyectan o se resuelven en tiempo de ejecución (a través de la interacción con un servicio, como el `LoanService`), sin que las clases (como `Person`) tengan que conocer directamente los detalles de las implementaciones concretas.

### En resumen:
- **Inversión de Dependencia**: Un principio SOLID que permite cambiar las dependencias en tiempo de ejecución y trabajar con abstracciones.
- **Composición sobre Herencia**: Preferencia por usar servicios y la composición en lugar de la herencia, para hacer el sistema más flexible y desacoplado.
- **Abstracción**: Usar traits o interfaces para que las clases trabajen con comportamientos genéricos y no con implementaciones concretas.

Este enfoque es útil para mantener un código más flexible, escalable y fácil de modificar sin afectar el resto del sistema.

### Terminología para referencia futura:
- **Inversión de Dependencias (Dependency Inversion)**
- **Composición sobre Herencia**
- **Desacoplamiento**
- **Abstracción**
  
Si te gustaría profundizar en alguna de estas áreas o ver más ejemplos, ¡dímelo! 😊