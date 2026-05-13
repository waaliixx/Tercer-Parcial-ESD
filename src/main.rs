/*
Segundo Parcial - Estructuras de Datos II
Alumno: Walter Ivan Vasquez Corvera VC25012
*/ 

#[derive(Debug, Clone)]
// Struct del libro con ISBN y título
struct Libro {
    isbn: u32,
    titulo: String,
}

// Nodo del arbol
struct Nodo {
    libro: Libro,
    izquierdo: Option<Box<Nodo>>,
    derecho: Option<Box<Nodo>>,
    altura: i32,
}

//implementación del nodo
impl Nodo {
    fn nuevo(libro: Libro) -> Self {
        Nodo {
            libro,
            izquierdo: None,
            derecho: None,
            altura: 1,
        }
    }
}

/* Funcion para obtener la altura de un nodo usando as_ref() y map_or()
 para manejar el caso de nodo None */
fn obtener_altura(nodo: &Option<Box<Nodo>>) -> i32 {
    nodo.as_ref().map_or(0, |n| n.altura)
}

// Funcion para actualizar la altura de un nodo
fn actualizar_altura(nodo: &mut Nodo) {
    nodo.altura = 1 + std::cmp::max(
        obtener_altura(&nodo.izquierdo),
        obtener_altura(&nodo.derecho),
    );
}

// Funcion para obtener el balance de un nodo
fn obtener_balance(nodo: &Nodo) -> i32 {
    obtener_altura(&nodo.izquierdo) - obtener_altura(&nodo.derecho)
}

/* Funcion de rotación a la derecha para mantener el balance del arbol, usando take() para mover
 la propiedad del nodo y evitar clones innecesarios*/
fn rotar_derecha(mut y: Box<Nodo>) -> Box<Nodo> {
    let mut x = y.izquierdo.take().expect("Hijo izquierdo ausente");
    y.izquierdo = x.derecho.take();
    actualizar_altura(&mut y);
    x.derecho = Some(y);
    actualizar_altura(&mut x);
    x
}

/* Funcion de rotación a la izquierda para mantener el balance del arbol, usando take() para mover
 la propiedad del nodo y evitar clones innecesarios*/
fn rotar_izquierda(mut x: Box<Nodo>) -> Box<Nodo> {
    let mut y = x.derecho.take().expect("Hijo derecho ausente");
    x.derecho = y.izquierdo.take();
    actualizar_altura(&mut x);
    y.izquierdo = Some(x);
    actualizar_altura(&mut y);
    y
}

/* Funcion para insertar un libro en el arbol, usando box para crear un nuevo nodo y manejar 
la propiedad de los nodos sin necesidad de clones*/
fn insertar(nodo_opt: Option<Box<Nodo>>, libro: Libro) -> Box<Nodo> {
    let mut nodo = match nodo_opt {
        None => return Box::new(Nodo::nuevo(libro)),
        Some(n) => n,
    };

    let isbn_nuevo = libro.isbn;

    if isbn_nuevo < nodo.libro.isbn {
        nodo.izquierdo = Some(insertar(nodo.izquierdo.take(), libro));
    } else if isbn_nuevo > nodo.libro.isbn {
        nodo.derecho = Some(insertar(nodo.derecho.take(), libro));
    } else {
        return nodo; 
    }

    actualizar_altura(&mut nodo);
    let balance = obtener_balance(&nodo);

    if balance > 1 && isbn_nuevo < nodo.izquierdo.as_ref().unwrap().libro.isbn {
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo > nodo.derecho.as_ref().unwrap().libro.isbn {
        return rotar_izquierda(nodo);
    }
    if balance > 1 && isbn_nuevo > nodo.izquierdo.as_ref().unwrap().libro.isbn {
        let hijo_izq = nodo.izquierdo.take().unwrap();
        nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
        return rotar_derecha(nodo);
    }
    if balance < -1 && isbn_nuevo < nodo.derecho.as_ref().unwrap().libro.isbn {
        let hijo_der = nodo.derecho.take().unwrap();
        nodo.derecho = Some(rotar_derecha(hijo_der));
        return rotar_izquierda(nodo);
    }
    nodo
}

/* Funcion para imprimir el arbol en orden, mostrando el ISBN y título de cada libro, 
con indentación para visualizar la estructura del árbol */
fn imprimir(nodo: &Option<Box<Nodo>>, nivel: usize) {
    if let Some(n) = nodo {
        imprimir(&n.derecho, nivel + 1);
        println!("{:indent$}[ISBN: {}] {}", "", n.libro.isbn, n.libro.titulo, indent = nivel * 4);
        imprimir(&n.izquierdo, nivel + 1);
    }
}

/* Funcion de búsqueda eficiente en el arbol AVL. Retorna una referencia al libro sin realizar copias.
   Realiza una búsqueda binaria aprovechando la estructura del árbol, con complejidad O(log n) */
fn buscar(nodo: &Option<Box<Nodo>>, isbn: u32) -> Option<&Libro> {
    match nodo {
        None => None,
        Some(n) => {
            if isbn == n.libro.isbn {
                Some(&n.libro)
            } else if isbn < n.libro.isbn {
                buscar(&n.izquierdo, isbn)
            } else {
                buscar(&n.derecho, isbn)
            }
        }
    }
}

/* Funcion auxiliar para encontrar el ISBN minimo (sucesor in-orden) en un subárbol.
   Se usa cuando el nodo a eliminar tiene dos hijos */
fn encontrar_minimo(nodo: &Box<Nodo>) -> u32 {
    if let Some(ref izq) = nodo.izquierdo {
        encontrar_minimo(izq)
    } else {
        nodo.libro.isbn
    }
}

/* Funcion para eliminar un libro del arbol AVL manteniendo el balance.
   Maneja 3 casos: nodo hoja, nodo con un hijo, nodo con dos hijos (usa sucesor in-orden).
   Tras eliminar, actualiza altura y realiza rotaciones si es necesario */
fn eliminar(nodo_opt: Option<Box<Nodo>>, isbn: u32) -> Option<Box<Nodo>> {
    match nodo_opt {
        None => None,
        Some(mut nodo) => {
            // Buscar el nodo a eliminar
            if isbn < nodo.libro.isbn {
                nodo.izquierdo = eliminar(nodo.izquierdo.take(), isbn);
            } else if isbn > nodo.libro.isbn {
                nodo.derecho = eliminar(nodo.derecho.take(), isbn);
            } else {
                // CASO 1: Nodo hoja (sin hijos)
                if nodo.izquierdo.is_none() && nodo.derecho.is_none() {
                    return None;
                }
                // CASO 2: Nodo con un hijo (solo hijo derecho)
                if nodo.izquierdo.is_none() {
                    return nodo.derecho.take();
                }
                // CASO 2: Nodo con un hijo (solo hijo izquierdo)
                if nodo.derecho.is_none() {
                    return nodo.izquierdo.take();
                }
                // CASO 3: Nodo con dos hijos - usar sucesor in-orden
                let sucesor_isbn = encontrar_minimo(nodo.derecho.as_ref().unwrap());
                if let Some(sucesor_libro) = buscar(&nodo.derecho, sucesor_isbn) {
                    nodo.libro = sucesor_libro.clone();
                }
                nodo.derecho = eliminar(nodo.derecho.take(), sucesor_isbn);
            }

            actualizar_altura(&mut nodo);
            let balance = obtener_balance(&nodo);

            if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) >= 0 {
                return Some(rotar_derecha(nodo));
            }
            if balance > 1 && obtener_balance(nodo.izquierdo.as_ref().unwrap()) < 0 {
                let hijo_izq = nodo.izquierdo.take().unwrap();
                nodo.izquierdo = Some(rotar_izquierda(hijo_izq));
                return Some(rotar_derecha(nodo));
            }
            if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) <= 0 {
                return Some(rotar_izquierda(nodo));
            }
            if balance < -1 && obtener_balance(nodo.derecho.as_ref().unwrap()) > 0 {
                let hijo_der = nodo.derecho.take().unwrap();
                nodo.derecho = Some(rotar_derecha(hijo_der));
                return Some(rotar_izquierda(nodo));
            }

            Some(nodo)
        }
    }
}

/* Funcion para contar el número total de nodos en el árbol de forma recursiva.
   Complejidad: O(n) donde n es el número de nodos */
fn contar_nodos(nodo: &Option<Box<Nodo>>) -> u32 {
    match nodo {
        None => 0,
        Some(n) => 1 + contar_nodos(&n.izquierdo) + contar_nodos(&n.derecho),
    }
}

/* Funcion para encontrar el libro con ISBN máximo en el árbol.
   En un árbol BST, el máximo siempre está en el nodo más a la derecha.
   Retorna una referencia al Libro sin copiar. Complejidad: O(h) donde h es altura */
fn encontrar_isbn_maximo(nodo: &Option<Box<Nodo>>) -> Option<&Libro> {
    match nodo {
        None => None,
        Some(n) => {
            if let Some(_) = &n.derecho {
                encontrar_isbn_maximo(&n.derecho)
            } else {
                Some(&n.libro)
            }
        }
    }
}

/* Struct para retornar estadísticas del árbol sin realizar copias innecesarias */
struct Estadisticas {
    altura: i32,
    total_nodos: u32,
    isbn_maximo: Option<String>, // String con formato "ISBN: {}, Titulo: {}"
}

/* Funcion que retorna todas las estadísticas del árbol en una sola llamada */
fn obtener_estadisticas(nodo: &Option<Box<Nodo>>) -> Estadisticas {
    let altura = obtener_altura(nodo);
    let total_nodos = contar_nodos(nodo);
    let isbn_maximo = encontrar_isbn_maximo(nodo).map(|libro| {
        format!("ISBN: {}, Título: {}", libro.isbn, libro.titulo)
    });
    
    Estadisticas {
        altura,
        total_nodos,
        isbn_maximo,
    }
}

fn main() {
    let mut raiz: Option<Box<Nodo>> = None;
    let datos = vec![
        (10, "El Quijote"), (20, "1984"), (30, "Hamlet"),
        (5, "Fahrenheit 451"), (2, "La Odisea"), (25, "El Principito"),
    ];

    println!("--- Sistema de Inventario de Librería (AVL) ---");
    for (isbn, titulo) in datos {
        let libro = Libro { isbn, titulo: titulo.to_string() };
        raiz = Some(insertar(raiz.take(), libro));
    }

    imprimir(&raiz, 0);
    
    // Estadísticas iniciales del árbol
    println!("\n--- Estadísticas del árbol inicial ---");
    let estadisticas = obtener_estadisticas(&raiz);
    println!("Altura total del árbol: {}", estadisticas.altura);
    println!("Total de nodos en el árbol: {}", estadisticas.total_nodos);
    if let Some(libro_max) = estadisticas.isbn_maximo {
        println!("Libro con ISBN más alto: {}", libro_max);
    }

    /* Estado original del arbol es:

            [ISBN: 30] Hamlet
                [ISBN: 25] El Principito
        [ISBN: 20] 1984
                [ISBN: 10] El Quijote
            [ISBN: 5] Fahrenheit 451
                [ISBN: 2] La Odisea

        el proceso de insercion y balanceo fue el siguiente:
        1. Insertar 10: Raiz es None, se crea nodo con 10.
        2. Insertar 20: Se inserta a la derecha de 10, altura de 10 se actualiza a 2, balance es -1 (ok).
        3. Insertar 30: Se inserta a la derecha de 20. Ahora el balance en la raíz (10) es -2 (altura derecha = 2, izquierda = 0). Como balance 
        < -1 y 30 > 20 (ISBN del hijo derecho), se realiza una rotación a la izquierda en el nodo raíz (10). El nodo 20 se convierte en la nueva raíz,
        y 10 pasa a ser hijo izquierdo de 20.
        4. Insertar 5: Se inserta a la izquierda de 20 (izquierda de 10). Sin rotación.
        5. Insertar 2: Se inserta a la izquierda de 5. Ahora el balance en el nodo 10 es 2 (altura izquierda = 2, derecha = 0). Como balance > 1 y 2 < 5 
        (ISBN del hijo izquierdo de 10), se realiza una rotación a la derecha en el nodo 10. El nodo 5 se convierte en hijo izquierdo de 20, y 10 pasa a ser hijo derecho de 5. 
        6. Insertar 25: Se inserta a la izquierda de 30. Sin rotación (balance en raíz = 0). 
        
         
        Analisis Explica en 5 líneas por qué es necesario usar .take() en las funciones de rotación en lugar de una asignación directa.

        R// En Rust, los valores dentro de un Box son propiedad única, lo que significa que no se pueden copiar ni clonar sin explícitamente hacerlo.
        Al usar .take(), movemos la propiedad del nodo hijo (izquierdo o derecho) a una variable temporal, lo que nos permite modificar la estructura
        del árbol sin violar las reglas de propiedad de Rust. Si intentáramos asignar directamente sin .take(), estaríamos intentando copiar el nodo, 
        lo cual no es permitido y resultaría en un error de compilación. Además, .take() deja el campo original como None, lo que es útil para evitar 
        referencias colgantes después de la rotación.
        */
    
    println!("\n--- Validación: Búsqueda de libros ---");
    
    // Prueba 1: Búsqueda exitosa
    match buscar(&raiz, 20) {
        Some(libro) => println!("Libro encontrado - ISBN: {}, Título: {}", libro.isbn, libro.titulo),
        None => println!("Libro con ISBN {} no encontrado", 20),
    }
    
    // Prueba 2: Búsqueda exitosa
    match buscar(&raiz, 5) {
        Some(libro) => println!("Libro encontrado - ISBN: {}, Título: {}", libro.isbn, libro.titulo),
        None => println!("Libro con ISBN {} no encontrado", 5),
    }
    
    // Prueba 3: Búsqueda exitosa
    match buscar(&raiz, 30) {
        Some(libro) => println!("Libro encontrado - ISBN: {}, Título: {}", libro.isbn, libro.titulo),
        None => println!("Libro con ISBN {} no encontrado", 30),
    }
    
    // Prueba 4: Búsqueda inexistente
    match buscar(&raiz, 100) {
        Some(libro) => println!("Libro encontrado - ISBN: {}, Título: {}", libro.isbn, libro.titulo),
        None => println!("Libro con ISBN {} no existe en el sistema", 100),
    }
    
    // Prueba 5: Búsqueda inexistente
    match buscar(&raiz, 15) {
        Some(libro) => println!("Libro encontrado - ISBN: {}, Título: {}", libro.isbn, libro.titulo),
        None => println!("Libro con ISBN {} no existe en el sistema", 15),
    }
    
    println!("\n--- Validación: Eliminación de libros ---");
    
    // Eliminación 1: Eliminar nodo hoja (ISBN 2)
    println!("\nEliminando ISBN 2 (nodo hoja)...");
    raiz = eliminar(raiz.take(), 2);
    match buscar(&raiz, 2) {
        Some(_) => println!("ISBN 2 aun existe (error)"),
        None => println!("ISBN 2 eliminado correctamente"),
    }
    
    // Eliminación 2: Eliminar nodo con un hijo (ISBN 5)
    println!("\nEliminando ISBN 5 (nodo con un hijo)...");
    raiz = eliminar(raiz.take(), 5);
    match buscar(&raiz, 5) {
        Some(_) => println!("ISBN 5 aun existe (error)"),
        None => println!("ISBN 5 eliminado correctamente"),
    }
    
    // Eliminación 3: Eliminar nodo con dos hijos (ISBN 20 - raiz actual)
    println!("\nEliminando ISBN 20 (nodo con dos hijos)...");
    raiz = eliminar(raiz.take(), 20);
    match buscar(&raiz, 20) {
        Some(_) => println!("ISBN 20 aun existe (error)"),
        None => println!("ISBN 20 eliminado correctamente"),
    }
    
    println!("\n--- Arbol despues de eliminaciones (debe estar balanceado) ---");
    imprimir(&raiz, 0);
    
    // Estadísticas finales del árbol
    println!("\n--- Estadísticas del árbol después de eliminaciones ---");
    let stats_final = obtener_estadisticas(&raiz);
    println!("Altura total del árbol: {}", stats_final.altura);
    println!("Total de nodos en el árbol: {}", stats_final.total_nodos);
    if let Some(libro_max) = stats_final.isbn_maximo {
        println!("Libro con ISBN más alto: {}", libro_max);
    } else {
        println!("El árbol está vacío");
    }
}
//```</Box<Nodo></Box<Nodo></Box<Nodo></Box<Nodo>
