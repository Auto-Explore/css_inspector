# css/CSS2/css1/c17-comments-000.xht

```json
{
  "format_version": 3,
  "file": "css/CSS2/css1/c17-comments-000.xht"
}
```

## style[0]

```css
<![CDATA[
   body { color: red; }
   /* This is a CSS comment. */
   .one {color: green;} /* Another comment */
   /* The following should not be used:
   .two {color: red;} */
   .three {color: green; /* color: red; */}
   /**
   .four {color: red;} */
   .five {color: green;}
   /**/
   .six {color: green;}
   /*********/
   .seven {color: green;}
   /* a comment **/
   .eight {color: green;}
  ]]>
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Invalid input.",
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
