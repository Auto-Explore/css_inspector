# css/css-fonts/variations/font-style-interpolation.html

```json
{
  "format_version": 3,
  "file": "css/css-fonts/variations/font-style-interpolation.html"
}
```

## style[0]

```css

        @keyframes fontStyleAnimation {
            from { font-style: oblique -12deg; }
            to   { font-style: oblique  12deg; }
        }

        #animation-test.animate {
            animation: fontStyleAnimation 1s infinite alternate;
        }

        #transition-test {
            font-style: oblique -12deg;
            transition-property: font-style;
            transition-duration: 10s;
        }

        #transition-test.animate {
            font-style: oblique 12deg;
        }

    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
