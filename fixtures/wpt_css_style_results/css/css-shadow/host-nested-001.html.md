# css/css-shadow/host-nested-001.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/host-nested-001.html"
}
```

## style[0]

```css

  #host {
    width: 100px;
    height: 100px;
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

      div {
        background: green;
        width: 100px;
        height: 100px;
      }
      :host { background: red !important }
      div:host { background: red !important }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
