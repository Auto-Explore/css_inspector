# css/css-variables/var-ident-function.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/var-ident-function.html"
}
```

## style[0]

```css

  #target {
    --myprop3:PASS;
    --var-with-ident-fn: FAIL1;
    --var-with-ident-fn: var(ident("--myprop" calc(3 * sign(1em - 1px))), FAIL2);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

  #target {
    --unparsed: ident("x");
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

  #target {
    --nodash: var(ident("nodash"));
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

  #target {
    --nodash-fallback: var(ident("nodash"), PASS);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[4]

```css

  :root {
    --nodash-fallback-inherit: PASS;
  }
  #target {
    --nodash-fallback-inherit: FAIL;
    --nodash-fallback-inherit: var(ident("nodash"), inherit);
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
