# css/css-mixins/mixin-media-query-invalidation.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-media-query-invalidation.html"
}
```

## style[0]

```css

        @mixin --m1() {
          @result {
            color: tomato;
          }
        }
        @media (width >= 50px) {
          @mixin --m1() {
            @result {
              color: cornsilk;
            }
          }
        }
        @media (width < 50px) {
          @mixin --m1() {
            @result {
              color: mediumvioletred;
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
