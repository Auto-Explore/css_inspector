# css/css-variables/css-vars-custom-property-inheritance.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/css-vars-custom-property-inheritance.html"
}
```

## style[0]

```css


        /* test cascade importance */
        :root { --color-1: green !important; }
        :root { --color-1: red; }
        div.color1 { background-color: var(--color-1); }

        /* test cascade order */
        :root { --color-2: green; }
        div.color2 { background-color: red; }
        div.color2 { background-color: var(--color-2); }

        div { width: 100px; height: 50px; }
    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background-color”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
