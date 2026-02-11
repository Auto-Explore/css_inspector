# css/css-view-transitions/view-transition-types-reserved-mutation.html

```json
{
  "format_version": 3,
  "file": "css/css-view-transitions/view-transition-types-reserved-mutation.html"
}
```

## style[0]

```css

  html:active-view-transition-type(foo) #positive1 {
    background: green;
  }

  html:active-view-transition-type(none, NoNe) #negative1 {
    background: red;
  }

  html:active-view-transition-type(-ua-foo, -UA-foo) #negative2 {
    background: red;
  }

  .test {
    width: 100px;
    height: 100px;
  }

  #positive1 {
    view-transition-name: positive1;
    background: red
  }

  #negative1 {
    view-transition-name: negative1;
    background: green
  }

  #negative2 {
    view-transition-name: negative2;
    background: green
  }

  #container {
    display: grid;
    grid-template-columns: repeat(3, 100px);
    grid-gap: 10px;
  }

  html::view-transition-group(*) {
    animation-play-state: paused;
  }

  html::view-transition-new(*) {
    animation: unset;
    opacity: 0;
  }

  html::view-transition-old(*) {
    animation: unset;
    opacity: 1;
  }

  html::view-transition-group(root) {
    display: none;
  }

  html::view-transition {
    background: red;
  }

  /* also test this type of construct */
  html:active-view-transition::view-transition {
    background: lightpink
  }
```

```json
{
  "errors": 13,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Unknown property “view-transition-name”.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
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
