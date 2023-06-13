#ifndef _INHERIT_ENUM_H_
#define _INHERIT_ENUM_H_

template <typename enumT, typename baseEnumT>
class InheritEnum {
    public:
        InheritEnum() {}
        InheritEnum(enumT e) : mEnum(e) {}
        InheritEnum(baseEnumT e) : mBaseEnum(e) {}
        explicit InheritEnum(int val) : mEnum(static_case<enumT>(val)) {}

        operator enumT() const { return mEnum; }
    private:
        union {
            enumT mEnum;
            baseEnumT baseEnumT;
        };
};

#endif // _INHERIT_ENUM_H_