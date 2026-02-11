# css/CSS2/positioning/relpos-calcs-004.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/relpos-calcs-004.xht"
}
```

## style[0]

```css

        div {
            height: 120px;
            width: 120px;
            margin-right: auto;
            direction: ltr;
        }
        .container {
            margin-left: -60px;
            direction: rtl; /* only set RTL on the containing block in question */
        }
        .outer {
            background: red;
            position: relative;
            left: 50%;
            /*
            div.outer's used left value is 60px
            div.outer's right top value is -60px
            div.outer's computed left value is 50%
            but
            div.outer's computed right value is auto and not 50%!
            */

        }
        .inner {
            background: green;
            position: relative;
            right: inherit; /* using inheritance to test computed vs. used */
        }
        .control {
          background: red;
          margin-bottom: -120px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
