# css/css-transitions/starting-style-name-defining-rules.html

```json
{
  "format_version": 3,
  "file": "css/css-transitions/starting-style-name-defining-rules.html"
}
```

## style[0]

```css

  @starting-style {
    @keyframes anim {
      from { color: green; }
      to { color: red; }
    }
    @font-face {
      font-family: web-font;
      src: url('/fonts/Ahem.ttf');
    }
    @layer first;
  }

  @layer second {
    #target.not_start {
      background-color: green;
    }
  }
  @layer first {
    #target.not_start {
      background-color: red;
    }
  }

  #target.not_start {
    animation-name: anim;
    animation-duration: 60s;
    animation-timing-function: step-end;
    font-family: web-font;
    font-size: 20px;
  }

  #target {
    /* For measuring text width */
    display: inline-block;
  }
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
