# css/css-nesting/nested-error-recovery.html

```json
{
  "format_version": 3,
  "file": "css/css-nesting/nested-error-recovery.html"
}
```

## style[0]

```css

  html {
    z-index: 0;

    error3234 {**** ::}

    #test1 {
      @media screen {
        z-index: 1;
      }
    }

    { whatever }

    :doesntexist {z-index: 9;}

    #test2 {
      z-index: 2;
    }

    #test3 {
      z-index: 3;
      @media screen {
        error3234;
        { foo }
        z-index:4;
      }
      { foo }
      z-index: 5;
    }

  }
```

```json
{
  "errors": 6,
  "messages": [
    {
      "message": "Invalid property name in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    },
    {
      "message": "Missing ':' in declaration.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
