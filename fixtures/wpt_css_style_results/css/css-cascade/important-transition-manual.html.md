# css/css-cascade/important-transition-manual.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/important-transition-manual.html"
}
```

## style[0]

```css

  .container { padding: 1em 0; border-style: dotted none; border-width: 1px; }
  .container > div { width: 5em; text-align: center; border: solid; transition: all 3s; }

  .container   > .ref  { border-color: blue ;           color: navy   ;           background: aqua ;           margin:   0.25em ;           }
  :hover       > .ref  { border-color: aqua ;           color: orange ;           background: teal ;           margin-left: 40% ;           }

  :not(:hover) > .test { border-color: blue !important; color: navy   !important; background: aqua ;                                        }
  div          > .test {                                                                                       margin:   0.25em !important; }
  :hover       > .test { border-color: aqua !important; color: orange ;           background: teal !important; margin-left: 40% !important; }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
