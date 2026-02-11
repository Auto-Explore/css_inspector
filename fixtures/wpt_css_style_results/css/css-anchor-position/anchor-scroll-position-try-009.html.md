# css/css-anchor-position/anchor-scroll-position-try-009.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scroll-position-try-009.html"
}
```

## style[0]

```css

body {
  margin: 0;
  width: 200vw;
  height: 200vh;
}

html {
  writing-mode: vertical-lr;
}

#anchor {
  anchor-name: --a;
  width: 100px;
  height: 100px;
  margin-block-start: 100vb;
  margin-inline-start: 100vi;
  background: orange;
}

#anchored {
  position: fixed;
  width: 100px;
  height: 100px;
  background: green;
  position-anchor: --a;
  position-try-fallbacks: --pf1, --pf2, --pf3;
  inset-block-start: anchor(end);
  inset-inline-start: anchor(end);
  position-visibility: always;
}

@position-try --pf1 {
  inset: auto;
  inset-block-end: anchor(start);
  inset-inline-start: anchor(end);
}
@position-try --pf2 {
  inset: auto;
  inset-block-start: anchor(end);
  inset-inline-end: anchor(start);
}
@position-try --pf3 {
  inset: auto;
  inset-block-end: anchor(start);
  inset-inline-end: anchor(start);
}
```

```json
{
  "errors": 6,
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
      "message": "Invalid value for property “background”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-anchor”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-try-fallbacks”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “position-visibility”.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
