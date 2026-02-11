# css/css-fonts/variations/slnt-backslant-variable.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/slnt-backslant-variable.html"
}
```

## style[0]

```css

  @font-face {
    font-family: "slnt test font";
    src: url('resources/FontStyleTest-slnt_backslant-VF.woff2');
    font-style: oblique -15deg 15deg;
  }

  .test {
    font-family: "slnt test font";
    font-size: 3em;
  }

  .backslant {
    font-style: oblique -14deg;
  }

  .oblique {
    font-style: oblique;
  }

  .italic {
    font-style: italic;
  }

```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “font-style”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
