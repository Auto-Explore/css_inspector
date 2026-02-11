# css/css-cascade/scope-proximity.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-proximity.html"
}
```

## style[0]

```css

  main * {
    background-color: black;
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

    .item {
      padding: 0px;
      border: 5px solid red;
    }

    @scope (.light) {
      [id] { border-color: rgb(100, 100, 100); }
    }

    @scope (.dark) {
      [id] { border-color: rgb(200, 200, 200); }
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

    @scope (.b) {
      [id] { border-color:green; }
    }
    @scope (.a) {
      [id] { border-color:red; }
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

    @scope (.a) {
      span[id] { border-color:green; }
    }
    @scope (.b) {
      [id] { border-color:red; }
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

    @scope (.foo) {
      .bar span[id] { border-color:green; }
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

    @scope (.scope) {
      :where(&) { border-color:green; }
    }
    @scope (#outer) {
      :where(:scope) :where(#inner) { border-color:red; }
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
