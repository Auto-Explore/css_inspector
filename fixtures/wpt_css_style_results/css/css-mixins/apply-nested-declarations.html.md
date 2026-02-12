# css/css-mixins/apply-nested-declarations.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/apply-nested-declarations.html"
}
```

## style[0]

```css

    @mixin --m1() {
      @result {
        color: green;
      }
    }
    #e1::after {
      content: "AFTER";
      @apply --m1;
    }
  
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[1]

```css

    @mixin --m2() {
      @result {
        color: red;
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
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```
