# css/css-borders/tentative/border-radius-side-shorthands/border-radius-side-shorthands-001.html

```json
{
  "format_version": 3,
  "file": "css/css-borders/tentative/border-radius-side-shorthands/border-radius-side-shorthands-001.html"
}
```

## style[0]

```css

div {
  width: 100px;
  height: 100px;
  margin: 10px;
}

#reference-shape-1,
#reference-shape-2,
#reference-shape-3,
#reference-shape-4 {
  border: 10px solid red;
}

#reference-shape-1 {
  border-top-left-radius: 40px;
  border-top-right-radius: 40px;
  border-bottom-right-radius: 20px;
  border-bottom-left-radius: 20px;
}

#reference-shape-2,
#reference-shape-4 {
  border-top-left-radius: 1em 4em;
  border-top-right-radius: 2em 3em;
  border-bottom-right-radius: 4em 1em;
  border-bottom-left-radius: 3em 2em;
}

#reference-shape-3 {
  border-top-left-radius: 40px;
  border-top-right-radius: 20px;
  border-bottom-right-radius: 20px;
  border-bottom-left-radius: 40px;
}

#test-1,
#test-2,
#test-3,
#test-4 {
  margin-top: -130px;
  border: 10px solid green;
}

#test-1 {
  border-top-radius: 40px;
  border-bottom-radius: 20px;
}

#test-2 {
  border-top-radius: 1em 4em / 2em 3em;
  border-bottom-radius: 3em 2em / 4em 1em;
}

#test-3 {
  border-left-radius: 40px;
  border-right-radius: 20px;
}

#test-4 {
  border-left-radius: 1em 4em / 3em 2em;
  border-right-radius: 2em 3em / 4em 1em;
}
```

```json
{
  "errors": 12,
  "messages": [
    {
      "message": "Invalid value for property “border-top-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-top-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “border-bottom-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-top-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-bottom-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-top-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-bottom-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-right-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-left-radius”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “border-right-radius”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
