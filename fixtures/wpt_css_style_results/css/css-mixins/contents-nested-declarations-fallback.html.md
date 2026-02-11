# css/css-mixins/contents-nested-declarations-fallback.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/contents-nested-declarations-fallback.html"
}
```

## style[0]

```css

    @mixin --m1() {
      @result {
        @contents {
          color: green;
        }
      }
    }
    #e1::after {
      content: "AFTER";
      @apply --m1;
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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

## style[1]

```css

    @mixin --m2() {
      @result {
        @contents {
          color: red;
        }
      }
    }
    /* Should match <div id=e2> with the specificity of :where(#e2) (zero),
       not with the specificity of :is(:where(#e2), #u1). */
    :where(#e2), #u1 {
      @apply --m2;
    }
    :where(#e2) {
      color: green; /* Wins. */
    }
  
```

```json
{
  "errors": 2,
  "messages": [
    {
      "message": "Unknown at-rule.",
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
