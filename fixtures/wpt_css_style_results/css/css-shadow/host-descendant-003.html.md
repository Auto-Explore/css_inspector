# css/css-shadow/host-descendant-003.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/host-descendant-003.html"
}
```

## style[0]

```css

my-host {
  display: block;
  width: 100px;
  height: 100px;
  background: red;
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

      :host > div {
        background-color: red;
        width: 100%;
        height: 100%;
      }
      :host > :not(span) {
        background-color: green;
      }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
