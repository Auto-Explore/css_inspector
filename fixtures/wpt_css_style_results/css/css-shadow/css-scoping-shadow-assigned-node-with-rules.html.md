# css/css-shadow/css-scoping-shadow-assigned-node-with-rules.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-assigned-node-with-rules.html"
}
```

## style[0]

```css

    my-host {
        display: block;
        width: 100px;
        height: 100px;
        overflow: hidden;
        background: green;
    }
    div {
        width: 100%;
        height: 50%;
    }
    .green {
        color: green;
    }
    .red {
        color: red;
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
 div { color: yellow; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
