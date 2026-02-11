# css/CSS2/syntax/case-sensitive-003.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/syntax/case-sensitive-003.xht"
}
```

## style[0]

```css

    * { color: red; }
    *:before, *:after { color: red; }

    p:FiRSt-cHIlD {
      color: green;
    }

    :LinK, :viSiTed {
      color: green;
      text-decoration: none;
      font: inherit;
    }

    :lAnG(en) {
      color: green;
    }

    .flin {
      white-space: nowrap;
    }
    .flin:fIrsT-LinE {
      color: green;
    }

    .flet span {
      color: green;
    }
    .flet:fIRst-LEttEr {
      color: green;
    }

    .bef:before, .af:after {
      content: "This sentence must be green.";
    }
    .bef:BeFoRe, .af:aFtEr {
      color: green;
    }

    /* test for ASCII (not UNICODE) case-insensitivity */
    p:fİrst-child {
      color: red;
    }
    p:fırst-child {
      color: red;
    }
    :lin&#x212a; {
      color: red;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid selector.",
      "severity": "Error"
    },
    {
      "message": "Invalid selector.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
