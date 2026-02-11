# css/css-variables/variable-pseudo-element.html

```json
{
  "format_version": 3,
  "file": "css/css-variables/variable-pseudo-element.html"
}
```

## style[0]

```css

        :root {
            --color: green;
            --color2: red;
        }
        div::before {
            content: '[before]';
        }
        div::after {
            content: '[after]';
        }

        #div1 {
            color: red;
        }
        #div1::before, #div1::after {
            color: var(--color2);
            --color2: green;
        }

        #div2 {
            color: var(--color2);
        }
        #div2::before, #div2::after {
            color: var(--color);
        }

        #div3 {
            color: var(--color);
        }
        #div3::before, #div3::after {
            --color: red;
        }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “color”.",
      "severity": "Error"
    },
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
