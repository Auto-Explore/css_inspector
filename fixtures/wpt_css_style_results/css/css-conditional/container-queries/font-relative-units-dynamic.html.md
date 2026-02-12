# css/css-conditional/container-queries/font-relative-units-dynamic.html

```json
{
  "format_version": 3,
  "file": "css/css-conditional/container-queries/font-relative-units-dynamic.html"
}
```

## style[0]

```css

  main {
    color: red;
  }
  #container {
    container-type: inline-size;
    width: 100px;
  }
  #container > div {
    font-size: 16px;
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

    main { font-size: 10px; }
    main.larger { font-size: 20px; }
    @container (width: 5em) {
      #test { color: green }
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

    :root { font-size: 10px; }
    :root.larger { font-size: 50px; }
    @container (width: 2rem) {
      #test { color: green }
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

    main { font-size: 10px; }
    main.larger { font-size: 20px; }
    @container (width <= 15ex) {
      #test { color: green }
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

    :root { font-size: 10px; }
    :root.larger { font-size: 20px; }
    @container (width <= 12rex) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css

    main { font-size: 10px; }
    main.larger { font-size: 20px; }
    @container (width <= 15ch) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[6]

```css

    main { font-family: 'Ahem'; font-size: 10px; }
    main.larger { font-size: 20px; }
    @container (width <= 7cap) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[7]

```css

    :root { font-size: 10px; }
    :root.larger { font-size: 20px; }
    @container (width <= 15rch) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[8]

```css

    main {
      font-size: 10px;
      line-height: 5;
    }
    main.larger { font-size: 20px; }
    @container (width <= 1lh) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[9]

```css

    :root {
      font-size: 10px;
      line-height: 5;
    }
    :root.larger {
      font-size: 20px;
    }
    @container (width <= 1rlh) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[10]

```css

    main { font-size: 10px; }
    main.larger { font-size: 20px; }
    @container (width <= 8ic) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[11]

```css

    :root { font-size: 10px; }
    :root.larger { font-size: 20px; }
    @container (width <= 8ric) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[12]

```css

    :root { font-family: 'Ahem'; font-size: 10px; }
    :root.larger { font-size: 20px; }
    @container (width <= 7rcap) {
      #test { color: green }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
