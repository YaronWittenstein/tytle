extern crate tytle;

use tytle::ast::semantic::{Procedure, Symbol, SymbolKind, SymbolTable, Variable};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sym_stable_starts_at_root_scope() {
        let mut table = SymbolTable::new();
        assert_eq!(table.is_root_scope(), true);
        assert_eq!(table.is_inner_scope(), false);

        table.start_scope();
        assert_eq!(table.is_root_scope(), false);
        assert_eq!(table.is_inner_scope(), true);

        table.end_scope();
        assert_eq!(table.is_root_scope(), true);
        assert_eq!(table.is_inner_scope(), false);
    }

    #[test]
    fn sym_table_one_scope_var_does_not_exist() {
        let mut table = SymbolTable::new();
        let scope = table.start_scope();
        let scope_id = scope.id;

        assert_eq!(None, table.lookup_symbol(scope_id, "A", &SymbolKind::Var));
    }

    #[test]
    fn sym_table_one_scope_var_exists() {
        let var = Variable::build_global("A");

        let mut table = SymbolTable::new();
        let scope = table.start_scope();
        let scope_id = scope.id;

        table.create_var_symbol(var.clone());

        assert_eq!(
            Symbol::Var(var),
            *table
                .lookup_symbol(scope_id, "A", &SymbolKind::Var)
                .unwrap()
        );
    }

    #[test]
    fn sym_table_one_scope_proc_exists() {
        let proc = Procedure::new("MYPROC");

        let mut table = SymbolTable::new();
        let scope = table.start_scope();
        let scope_id = scope.id;

        table.create_proc_symbol(proc.clone());

        assert_eq!(
            Symbol::Proc(proc),
            *table
                .lookup_symbol(scope_id, "MYPROC", &SymbolKind::Proc)
                .unwrap()
        );
    }

    #[test]
    fn sym_table_one_scope_var_and_proc_with_the_same_name() {
        let var = Variable::build_global("A");
        let proc = Procedure::new("A");

        let mut table = SymbolTable::new();
        let scope = table.start_scope();
        let scope_id = scope.id;

        table.create_var_symbol(var.clone());
        table.create_proc_symbol(proc.clone());

        assert_eq!(
            Symbol::Var(var),
            *table
                .lookup_symbol(scope_id, "A", &SymbolKind::Var)
                .unwrap()
        );

        assert_eq!(
            Symbol::Proc(proc),
            *table
                .lookup_symbol(scope_id, "A", &SymbolKind::Proc)
                .unwrap()
        );
    }

    #[test]
    fn sym_multiple_nested_scopes_inner_scope_var_exists_while_shadowing_an_outer_scope_var() {
        //
        // Scope outer
        // |
        // | variable A=100 (outer)
        // |
        // |---- Scope inner
        //     |
        //     | variable A=200 (inner)
        //     |

        let mut table = SymbolTable::new();

        // outer scope
        let outer_scope = table.start_scope();
        let outer_scope_id = outer_scope.id;
        let mut var_outer = Variable::build_local("A");
        var_outer.set_reference(100);
        table.create_var_symbol(var_outer.clone());

        let mut var_inner = Variable::build_local("A");
        var_inner.set_reference(200);
        let inner_scope = table.start_scope();
        let inner_scope_id = inner_scope.id;
        table.create_var_symbol(var_inner.clone());

        assert_eq!(outer_scope_id, 1);
        assert_eq!(inner_scope_id, 2);

        assert_eq!(
            Symbol::Var(var_inner),
            *table
                .lookup_symbol(inner_scope_id, "A", &SymbolKind::Var)
                .unwrap()
        );

        assert_eq!(
            Symbol::Var(var_outer),
            *table
                .lookup_symbol(outer_scope_id, "A", &SymbolKind::Var)
                .unwrap()
        );
    }

    #[test]
    fn sym_table_multiple_nested_scopes_var_does_exist_on_parent_scope() {
        //
        // Scope X
        // |
        // | variable A=100
        // |
        // |---- Scope Y
        //     |
        //     |
        //     |---- Scope Z

        let mut table = SymbolTable::new();

        // scope X
        let scope_x = table.start_scope();
        let scope_x_id = scope_x.id;

        // var
        let mut var = Variable::build_local("A");
        var.set_reference(100);
        table.create_var_symbol(var.clone());

        // scope Y
        let scope_y = table.start_scope();
        let scope_y_id = scope_y.id;

        // scope Z
        let scope_z = table.start_scope();
        let scope_z_id = scope_z.id;

        assert_eq!(
            Symbol::Var(var.clone()),
            *table
                .recursive_lookup_sym(scope_z_id, "A", &SymbolKind::Var)
                .unwrap()
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *table
                .recursive_lookup_sym(scope_y_id, "A", &SymbolKind::Var)
                .unwrap()
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *table
                .recursive_lookup_sym(scope_x_id, "A", &SymbolKind::Var)
                .unwrap()
        );
    }

    #[test]
    fn sym_table_multiple_nested_scopes_var_does_not_exist_at_any_scope() {
        //
        // Scope X
        // |
        // |---- Scope Y
        //     |
        //     |---- Scope Z

        let mut table = SymbolTable::new();
        table.start_scope(); // scope X
        table.start_scope(); // scope Y
        let scope_z = table.start_scope(); // scope Z
        let scope_z_id = scope_z.id;

        assert_eq!(
            None,
            table.recursive_lookup_sym(scope_z_id, "A", &SymbolKind::Var)
        );
    }

    #[test]
    fn sym_table_multiple_not_nested_scopes_var_exist_under_exactly_one_scope() {
        //
        // Scope X
        // |
        // |------
        //
        //  Scope Y
        // |
        // | variable A (reference=100)
        // |----
        //
        // Scope Z
        // |
        // |------

        let mut table = SymbolTable::new();

        // scope X
        let scope_x = table.start_scope(); // scope X
        let scope_x_id = scope_x.id;
        table.end_scope();

        // scope Y
        let scope_y = table.start_scope(); // scope Y
        let scope_y_id = scope_y.id;
        let mut var = Variable::build_local("A");
        var.set_reference(100);
        table.create_var_symbol(var.clone());
        table.end_scope();

        // scope Z
        let scope_z = table.start_scope(); // scope Z
        let scope_z_id = scope_z.id;
        table.end_scope();

        assert_eq!(scope_x_id, 1);
        assert_eq!(scope_y_id, 2);
        assert_eq!(scope_z_id, 3);

        assert_eq!(
            None,
            table.recursive_lookup_sym(scope_x_id, "A", &SymbolKind::Var)
        );
        assert_eq!(
            None,
            table.recursive_lookup_sym(scope_z_id, "A", &SymbolKind::Var)
        );
        assert_eq!(
            Symbol::Var(var.clone()),
            *table
                .recursive_lookup_sym(scope_y_id, "A", &SymbolKind::Var)
                .unwrap()
        );
    }

    #[test]
    fn sym_table_get_current_scope() {
        //
        // Scope X
        // |
        // |---- Scope Y
        //     |
        //     |---- Scope Z

        let mut table = SymbolTable::new();

        // root scope
        let root_scope = table.get_current_scope();
        assert_eq!(None, root_scope);

        // scope X
        let scope_x = table.start_scope();
        let scope_x_id = scope_x.id;

        let scope = table.get_current_scope().unwrap();
        assert_eq!(scope_x_id, scope.id);

        // scope Y
        let scope_y = table.start_scope();
        let scope_y_id = scope_y.id;

        let scope = table.get_current_scope().unwrap();
        assert_eq!(scope_y_id, scope.id);
        table.end_scope(); // closing `scope Y`, back to `scope X`

        // we're again under `scope X`
        let scope = table.get_current_scope().unwrap();
        assert_eq!(scope_x_id, scope.id);

        table.end_scope(); // closing `scope X`, back to `root scope`
        let root_scope = table.get_current_scope();
        assert_eq!(None, root_scope);
    }
}