# css/css-writing-modes/orthogonal-child-with-border.html

```json
{
  "format_version": 3,
  "file": "css/css-writing-modes/orthogonal-child-with-border.html"
}
```

## style[0]

```css

    #outer {
      position: absolute;
      background: red;
      writing-mode: vertical-lr;
    }
    #inner {
      width: 100px;
      border: 0px solid green;
      border-width: 40px 20px 60px 30px;  /* top + bottom borders = 100px */
      writing-mode: horizontal-tb;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
