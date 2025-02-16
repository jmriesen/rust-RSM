import XCTest
import SwiftTreeSitter
import TreeSitterMumps

final class TreeSitterMumpsTests: XCTestCase {
    func testCanLoadGrammar() throws {
        let parser = Parser()
        let language = Language(language: tree_sitter_mumps())
        XCTAssertNoThrow(try parser.setLanguage(language),
                         "Error loading Mumps grammar")
    }
}
