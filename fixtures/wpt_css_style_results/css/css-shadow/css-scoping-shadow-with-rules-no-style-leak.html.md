# css/css-shadow/css-scoping-shadow-with-rules-no-style-leak.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-with-rules-no-style-leak.html"
}
```

## style[0]

```css

    my-host {
        display: block;
    }
    div {
        width: 100px;
        height: 100px;
        background: green;
        color:green;
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
 div { background: red; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
