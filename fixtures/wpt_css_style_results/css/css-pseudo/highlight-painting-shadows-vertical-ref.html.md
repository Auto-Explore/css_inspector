# css/css-pseudo/highlight-painting-shadows-vertical-ref.html

```json
{
  "format_version": 3,
  "file": "css/css-pseudo/highlight-painting-shadows-vertical-ref.html"
}
```

## style[0]

```css

    :root {
        font-family: Ahem;
        writing-mode: vertical-lr;
        line-height: 1;
        white-space: break-spaces;
    }
    #originating_shadow {
        color: transparent;
        text-shadow: 0.1em 0.1em rgba(0,0,0,0.5);
        position: absolute;
        z-index: 0;
        top: 10px;
        left: 10px;
    }
    #originating_text {
        color: black;
        position: absolute;
        z-index: 7;
        top: 10px;
        left: 10px;
    }
    #selection_only {
        color: green;
        text-shadow: -0.25em -0.25em rgba(0,128,0,0.5), 0.1em 0.1em rgba(0,0,0,0.5);
        position: absolute;
        z-index: 5;
        left: 10px;
    }
    #target {
        color: blue;
        text-shadow: 0.25em 0.25em rgba(0,0,128,0.5), 0.1em 0.1em rgba(0,0,0,0.5);
        position: absolute;
        z-index: 3;
        left: 10px;
    }
    #both_text {
        color: green;
        position: absolute;
        z-index: 4;
        left: 10px;
    }
    #both_shadow {
        color: transparent;
        text-shadow: 0.25em 0.25em rgba(0,0,128,0.5), -0.25em -0.25em rgba(0,128,0,0.5), 0.1em 0.1em rgba(0,0,0,0.5);
        position: absolute;
        z-index: 2;
        left: 10px;
    }
```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    },
    {
      "message": "Invalid value for property “text-shadow”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
