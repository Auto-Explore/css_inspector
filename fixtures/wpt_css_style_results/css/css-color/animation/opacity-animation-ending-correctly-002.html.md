# css/css-color/animation/opacity-animation-ending-correctly-002.html

```json
{
  "format_version": 3,
  "file": "css/css-color/animation/opacity-animation-ending-correctly-002.html"
}
```

## style[0]

```css

#test {
  position: sticky;
  top: 0;
  height: 50px;
  background: blue;
  transform: translate(0);
  filter: grayscale(0%);
  transition: opacity 50ms step-start;
}

#test.fade {
  opacity: 0.2;
}

.tall {
  height: 5000px;
}
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Invalid value for property “filter”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
