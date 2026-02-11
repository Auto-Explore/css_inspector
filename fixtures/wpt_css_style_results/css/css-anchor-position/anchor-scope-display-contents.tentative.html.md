# css/css-anchor-position/anchor-scope-display-contents.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-anchor-position/anchor-scope-display-contents.tentative.html"
}
```

## style[0]

```css

  .scope-all { anchor-scope: all; display: contents; }
  .scope-a { anchor-scope: --a; display: contents; }
  .scope-b { anchor-scope: --b; ; display: contents; }
  .scope-ab { anchor-scope: --a, --b; ; display: contents; }

  .anchor-a { anchor-name: --a; }
  .anchor-b { anchor-name: --b; }
  .anchor-ab { anchor-name: --a, --b; }
  .anchor-a, .anchor-b, .anchor-ab {
    background: skyblue;
    height: 10px;
  }

  .anchored-a { position-anchor: --a; }
  .anchored-b { position-anchor: --b; }
  .anchored-a, .anchored-b {
    position: absolute;
    top: anchor(bottom);
    left: anchor(left);
    width: 5px;
    height: 5px;
    background: coral;
  }

  /* Containing block */
  main {
    position: relative;
    width: 100px;
    height: 100px;
    border: 1px solid black;
  }
```

```json
{
  "errors": 11,
  "messages": [
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “anchor-scope”.",
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
      "message": "Unknown property “position-anchor”.",
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

## style[1]

```css

    .anchor-a, .anchor-b {
      position: absolute;
      width: 5px;
      height: 5px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[2]

```css

    .abs {
      position: absolute;
      width: 5px;
      height: 5px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[3]

```css

    .abs {
      position: absolute;
      width: 5px;
      height: 5px;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
