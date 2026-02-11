# css/css-writing-modes/orthogonal-child-with-padding.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/orthogonal-child-with-padding.html"
}
```

## style[0]

```css

    #outer {
      position: absolute;
      background: red;
    }
    #inner {
      height: 50px;
      min-width: 50px;
      padding-left: 25px;
      padding-right: 25px;
      writing-mode: vertical-rl;
    }
    #mask {
      position: absolute;
      width: 100px;
      height: 100px;
      background: green;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
