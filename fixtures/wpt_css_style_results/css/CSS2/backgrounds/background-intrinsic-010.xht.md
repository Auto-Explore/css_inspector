# css/CSS2/backgrounds/background-intrinsic-010.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-intrinsic-010.xht"
}
```

## style[0]

```css

    /* Setup. Use 5:6 ratio because it's weird and unlikely to be hard-coded anywhere. */
    div {
      position: relative;
    }
    .cover, .limit {
      width: 120px;
      height: 120px;
      margin: 0.5em;
      background: green; /* Used to match reference; remove for debugging. */
    }
    .control {
      position: absolute;
      top: 10px; bottom: 10px;
      left: 10px; right: 10px;
    }
    .cover .control {
      background: red;
    }
    .limit .control {
      background: green;
    }
    .test {
      /* 80x100 bgpos area */
      height: 80px;
      width: 60px;
      padding: 10px;
      border: 10px solid transparent;
    }

    /* Test */
    .cover .test {
      background: no-repeat url(support/green-portrait.png);
    }
    .limit .test {
      background: no-repeat url(support/red-portrait.png);
    }
    .control {
      width: 40px;
      height: 60px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
