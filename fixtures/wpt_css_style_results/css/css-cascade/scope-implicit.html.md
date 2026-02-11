# css/css-cascade/scope-implicit.html

```json
{
  "format_version": 3,
  "file": "css/css-cascade/scope-implicit.html"
}
```

## style[0]

```css

      @scope {
        .a { z-index:1; }
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

        @scope {
          :scope { z-index:1; }
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

      @scope {
        .a { z-index:1; }
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

      @scope {
        .a { z-index:1; }
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

      @scope ($invalid) {
        #a { z-index:1; }
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

      @scope {
        #div { z-index:1; }
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

      @scope {
        > div { z-index:1; }
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

      @scope {
        /* Behaves like :scope */
        & { z-index:1; }
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

## style[8]

```css

      @scope to (.b) {
        .a { z-index:1; }
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

## style[9]

```css

@scope {
  * { z-index: 1;}
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

## style[10]

```css

      @scope {
        * { z-index:2; }
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

## style[11]

```css

@scope {
  * { z-index: 3;}
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

## style[12]

```css

        @scope (#outer) {
          @scope {
            @scope (#inner) {
              :scope {
                z-index: 1;
              }
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

## style[13]

```css

      @scope (#outer) {
        @scope {
          :scope {
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
