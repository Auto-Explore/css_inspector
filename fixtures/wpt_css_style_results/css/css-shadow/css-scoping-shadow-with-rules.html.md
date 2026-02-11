# css/css-shadow/css-scoping-shadow-with-rules.html

```json
{
  "format_version": 3,
  "file": "css/css-shadow/css-scoping-shadow-with-rules.html"
}
```

## style[0]

```css

    my-host {
        display: block;
        width: 100px;
        height: 100px;
    }
    div {
        width: 100%; height: 100%; background: red;
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
 div {width: 100px; height: 100px; background: green; color:green; } 
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
