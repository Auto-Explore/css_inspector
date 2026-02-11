# css/css-contain/content-visibility/content-visibility-anchor-positioning-007.html

```json
{
  "format_version": 3,
  "file": "css/css-contain/content-visibility/content-visibility-anchor-positioning-007.html"
}
```

## style[0]

```css

@position-try --foo {
  right: anchor(--a2 left);
}

#container {
  position:relative;
  height: 10000px;
}

#lock {
  height: 400px;
  width: 400px;
  content-visibility: auto;
  border: 1px solid black;
}


#anchor {
  anchor-name: --a1;
  position: absolute;
}

#anchor2 {
  anchor-name: --a2;
  position: absolute;
  left: 60px;
}

#positioned {
  position: absolute;
  right: anchor(--a1 left);
  position-try: --foo;
}

```

```json
{
  "errors": 4,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
