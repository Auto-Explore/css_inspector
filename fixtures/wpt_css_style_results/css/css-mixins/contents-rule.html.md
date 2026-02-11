# css/css-mixins/contents-rule.html

```json
{
  "format_version": 3,
  "file": "css/css-mixins/contents-rule.html"
}
```

## style[0]

```css

      @mixin --m1() {
        @result {
          @contents;
        }
      }
      #e1 {
        color: red;
        @apply --m1 { color: green; }
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

      @mixin --m2() {
        @result {
          @contents
        }
      }
      #e2 {
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

## style[2]

```css

      @mixin --m3() {
        @result {
          &.a {
            @contents { color: blue; }
          }
        }
      }
      .b {
        color: red;
        @apply --m3 { color: green; }
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

      @mixin --m4() {
        @result {
          &.c {
            @contents { color: green; }
          }
        }
      }
      .d {
        color: red;
        @apply --m4;
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

## style[4]

```css

      @mixin --m6() {
        @result {
          @contents { }
          color: green;
        }
      }
      #e6 {
        @apply --m6;
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

## style[5]

```css

        @mixin --m7() {
          @result {
            @contents;
            color: green;
          }
        }
        #e7 {
          @apply --m7 {};
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
