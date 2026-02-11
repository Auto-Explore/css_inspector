# css/css-fonts/variations/font-weight-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/font-weight-interpolation.html"
}
```

## style[0]

```css

        @keyframes fontWeightAnimation {
            from { font-weight: 100; }
            to   { font-weight: 900; }
        }

        #animation-test.animate {
            animation: fontWeightAnimation 1s infinite alternate;
        }

        #transition-test {
            font-weight: 100;
            transition-property: font-weight;
            transition-duration: 10s;
        }

        #transition-test.animate {
            font-weight: 900;
        }

    
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “animation”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
