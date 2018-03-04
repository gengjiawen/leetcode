// https://leetcode.com/problems/reverse-words-in-a-string-iii/description/

// Given a string, you need to reverse the order of characters in each word within a sentence while still preserving whitespace and initial word order.

// Example 1:

// Input: "Let's take LeetCode contest"
// Output: "s'teL ekat edoCteeL tsetnoc"

// Note: In the string, each word is separated by single space and there will not be any extra space in the string.
import "regexp"
func reverseWords(s string) string {
    res := func (s string) string {
		var c = strings.Split(s, "")

		for i, j := 0, len(c)-1; i < j; i, j = i+1, j-1 {
			c[i], c[j] = c[j], c[i]
		}

		return strings.Join(c, "")
	}

	r := regexp.MustCompile("[^\\s]+")
	words := r.FindAllString(s, -1)

	for i, v := range words {
		words[i] = res(v)
	}

	return strings.Join(words, " ")
}

