# css/CSS2/positioning/relpos-calcs-005.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/positioning/relpos-calcs-005.xht"
}
```

## style[0]

```css

        div {
            height: 120px;
            width: 80px;
            direction: rtl;
            margin-right: auto;
        }
        .container {
            direction: ltr; /* only set LTR on the containing block in question */
        }
        .outer {
            background: green;
            position: relative;
            left: 50%;
            right: 50%;

            /*
            div.outer's used left value is 60px
            div.outer's used right value is -60px
            div.outer's computed left value is 50%
            div.outer's computed right value is 50%
            */

        }
        .inner {
            background: green;
            position: relative;
            right: inherit; /* using inheritance to test computed vs. used */
            /* div.inner's right inherits div.outer's computed right value */
        }
        .control {
          background: red;
          margin-bottom: -120px;
          width: 120px;
        }
    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
