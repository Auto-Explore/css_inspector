# css/css-images/gradient/gradient-analogous-missing-components-002-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-images/gradient/gradient-analogous-missing-components-002-ref.html"
}
```

## style[0]

```css

        .test {
            margin: 10px 50px;
            width: 200px;
            height: 50px;
            border: 1px solid black;
            --color: lime;
        }

        .test1 {
            background: linear-gradient(to right, var(--color));
        }
        .test2 {
            background: linear-gradient(to right in srgb, var(--color));
        }
        .test3 {
            background: linear-gradient(to right in oklab, var(--color));
        }
        .test4 {
            background: linear-gradient(to right in display-p3, var(--color));
        }
    
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
