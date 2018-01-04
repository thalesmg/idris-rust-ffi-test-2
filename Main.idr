module Main

%link C "libkabum.so"

soma_marota : Int -> IO Int
soma_marota = foreign FFI_C "soma_marota" (Int -> IO Int)

string_marota : IO (Raw String)
string_marota = foreign FFI_C "string_marota" (IO (Raw String))

imprimir_maroto : (Raw String) -> IO ()
imprimir_maroto = foreign FFI_C "imprimir_maroto" ((Raw String) -> IO ())

sanity_check : IO ()
sanity_check = foreign FFI_C "sanity_check" (IO ())

main : IO ()
main = do
  sanity_check
  putStrLn "Vou somar"
  res <- soma_marota 53
  putStrLn ("Resultado: " ++ show res)
  stringue <- string_marota
  imprimir_maroto stringue
