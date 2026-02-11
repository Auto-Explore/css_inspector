# css/css-mixins/mixin-cssom.tentative.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/mixin-cssom.tentative.html"
}
```

## style[0]

```css

      @mixin --m1() {
        @result {
        color: green;
        & {
          --foo: bar;
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

## style[1]

```css

       #foo {
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

## style[2]

```css

      @mixin --m2() {
        @result {
          @contents
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

      #foo {
        color: red;
        @apply --m2 { color: green; }
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

    
```

```json
{
  "errors": 0,
  "messages": [],
  "warnings": 0
}
```

## style[5]

```css

      @mixin --m3(--arg type(<length>): 1em, --other-arg) {
        @result {
          margin-left: var(--arg);
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
