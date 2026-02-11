# css/css-nesting/nesting-layer.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/nesting-layer.html"
}
```

## style[0]

```css


  .a {
    /* This should have no effect. Only at-rules containing style rules
       are vaild when nested. */
    @layer theme, base;
  }

  /* The theme layer wins over the base layer. */
  @layer base, theme;

  .a {
    @layer theme {
      & {
        z-index: 1;
      }

      .b {
        background-color: green;
      }
    }
  }

  @layer base {
    .a {
      z-index: 0;
    }
    .a .b {
      background-color: red;
    }
  }
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
