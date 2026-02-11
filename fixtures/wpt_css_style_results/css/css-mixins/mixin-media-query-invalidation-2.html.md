# css/css-mixins/mixin-media-query-invalidation-2.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-media-query-invalidation-2.html"
}
```

## style[0]

```css

        @mixin --m1() {
          @result {
            color: red;
          }
        }
        @media (width < 50px) {
          @mixin --m1() {
            @result {
              color: green;
            }
          }
        }
      
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
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[1]

```css

        /* Invalidated by a mixin in a different style sheet. */
        #target {
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
