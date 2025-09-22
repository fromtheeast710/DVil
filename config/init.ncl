let rec even_index
  : forall a. Array a -> Array a
  = match {
    l @ ([] or [_]) => l,
    [h, _, ..t] => [h] @ even_index t,
  }
in even_index [0, 1, 2, 3, 4, 5, 6]
