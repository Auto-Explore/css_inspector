# css/CSS2/backgrounds/background-intrinsic-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/backgrounds/background-intrinsic-003.xht"
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
      left: 10px; right: 30px;
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
      background: no-repeat url(support/green-intrinsic-height.svg);
    }
    .limit .test {
      background: no-repeat url(support/red-intrinsic-height.svg);
    }
    .control {
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
