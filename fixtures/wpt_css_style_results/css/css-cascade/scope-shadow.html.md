# css/css-cascade/scope-shadow.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-shadow.html"
}
```

## style[0]

```css

      @scope (:host) {
        .a {
          z-index: 1;
        }
      }
    
```

```json
{
  "errors": 1,
  "messages": [
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

      @scope (:host(div)) {
        .a {
          z-index: 1;
        }
      }
      /* Should not match: */
      @scope (:host(span)) {
        .a {
          z-index: 42;
        }
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[2]

```css

      @scope (:host) {
        :scope {
          z-index: 1;
        }
      }
    
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[3]

```css

        /* Should not match host, nor outside shadow. */
        :is(:scope, .a, .host) {
          z-index: 2;
        }

        @scope (:host) {
          :is(:scope, .a) {
            z-index: 1;
          }
        }
      
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[4]

```css

          @scope {
            /* Matches host */
            :scope {
              z-index: 1;
            }
            :scope > .a {
              z-index: 2;
            }
          }
      
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[5]

```css

          @scope (:host) {
            :scope {
              & {
                z-index: 1;
              }
            }
          }
      
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[6]

```css

          @scope (:host) {
            :scope .a {
              & {
                z-index: 1;
              }
            }
          }
      
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```

## style[7]

```css

          @scope (:host) {
            :scope > .a {
              & {
                z-index: 1;
              }
            }
          }
      
```

```json
{
  "errors": 1,
  "messages": [
    {
      "message": "Unknown at-rule.",
      "severity": "Error"
    }
  ],
  "warnings": 0
}
```
